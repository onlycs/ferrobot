use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert,
    ffi::c_void,
    fmt,
    sync::LazyLock,
};

use async_std::sync::RwLock;
use futures::future::{self, BoxFuture};

use crate::{device::Device, prelude::*};

// to make a future send-sync even when rust says it isn't
// (i used a raw ptr that one time and it crashes out)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ArcPtr {
    ptr: *const c_void,
}

impl ArcPtr {
    fn new<T>(arc: Arc<T>) -> Self {
        Self {
            ptr: Arc::into_raw(arc).cast(),
        }
    }

    unsafe fn to_arc<T>(self) -> Arc<T> {
        unsafe { Arc::from_raw(self.ptr.cast()) }
    }
}

unsafe impl Send for ArcPtr {}
unsafe impl Sync for ArcPtr {}

type ErasedArc = Arc<dyn Any + Send + Sync>;
type AsyncCallback = Arc<dyn (Fn(ErasedArc) -> BoxFuture<'static, ()>) + Send + Sync>;
type CallbackMap = HashMap<TypeId, HashMap<ArcPtr, Vec<AsyncCallback>>>;

pub(crate) struct Emitter {
    callbacks: Arc<RwLock<CallbackMap>>,
}

impl Emitter {
    pub(crate) fn instance() -> &'static Emitter {
        static INSTANCE: LazyLock<Arc<Emitter>> = LazyLock::new(|| Arc::new(Emitter::new()));
        &INSTANCE
    }

    fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) async fn register<E: Event + 'static>(
        &self,
        event: &Arc<E>,
        callback: Arc<dyn (Fn(Arc<E::Data>) -> BoxFuture<'static, ()>) + Send + Sync>,
    ) {
        let type_id = TypeId::of::<E>();
        let event_ptr = ArcPtr::new(Arc::clone(event));
        let callback = Arc::new(move |data: Arc<dyn Any + Send + Sync>| {
            callback(unsafe { Arc::downcast_unchecked(data) })
        }) as AsyncCallback;

        let mut callbacks = self.callbacks.write().await;
        let entry = callbacks.entry(type_id).or_default();

        if let Some(callbacks) = entry.get_mut(&event_ptr) {
            callbacks.push(callback);
            unsafe { drop(event_ptr.to_arc::<E>()) } // this ptr can die, since we already leaked it
        } else {
            entry.insert(event_ptr, vec![callback]); // we can leak event here, since this ptr needs to live
        }
    }

    pub(crate) async fn register_trigger<Tr: Event, Dst: Event>(
        &self,
        trigger: &Arc<Tr>,
        event: Arc<Dst>,
        map: fn(Arc<Tr::Data>) -> Arc<Dst::Data>,
    ) {
        self.register(
            trigger,
            Arc::new(move |data: Arc<Tr::Data>| {
                let event = Arc::clone(&event);
                let data = map(data);
                Box::pin(Emitter::instance().emit(event, data))
            }),
        )
        .await;
    }

    async fn emit<E: Event + 'static>(&self, event: Arc<E>, data: Arc<E::Data>) {
        let callbacks = self.callbacks.read().await;
        let type_id = TypeId::of::<E>();
        let event_ptr = ArcPtr::new(event);
        let data = data as Arc<dyn Any + Send + Sync>;

        if let Some(entry) = callbacks.get(&type_id) {
            if let Some(callbacks) = entry.get(&event_ptr) {
                future::join_all(callbacks.iter().map(|callback| callback(Arc::clone(&data))))
                    .await;
            }
        }

        // drop event_ptr, leak bad
        unsafe { drop(event_ptr.to_arc::<E>()) }
    }

    pub(crate) async fn emit_device<D: Device + 'static>(&self, event: Arc<D>, data: Arc<D::Data>) {
        self.emit(Arc::clone(&event), data).await;
    }
}

unsafe impl Send for Emitter {}
unsafe impl Sync for Emitter {}

pub trait Event: Sized + Send + Sync + 'static {
    type Data: Send + Sync + 'static;
}

pub trait EventExt: Event {
    async fn register_fallible<
        E: fmt::Debug + Send + 'static,
        Fut: Future<Output = Result<(), E>> + Send + 'static,
        F: Fn(Arc<Self::Data>) -> Fut + Send + Sync + 'static,
    >(
        self: &Arc<Self>,
        f: F,
        on_err: fn(E),
    ) {
        Emitter::instance()
            .register(
                self,
                Arc::new(move |data| {
                    let fut = f(data);
                    Box::pin(async move { fut.await.unwrap_or_else(on_err) })
                }),
            )
            .await;
    }

    async fn register<
        Fut: Future<Output = ()> + Send + 'static,
        F: Fn(Arc<Self::Data>) -> Fut + Send + Sync + 'static,
    >(
        self: &Arc<Self>,
        f: F,
    ) {
        Emitter::instance()
            .register(self, Arc::new(move |data| Box::pin(f(data))))
            .await;
    }

    async fn trigger<E: Event<Data = Self::Data>>(self: &Arc<Self>, other: &Arc<E>) {
        Emitter::instance()
            .register_trigger(self, Arc::clone(other), convert::identity)
            .await;
    }
}

impl<T: Event> EventExt for T {}

impl<D: Device> Event for D {
    type Data = D::Data;
}
