use std::{collections::HashMap, ffi::c_void, sync::LazyLock};

use async_std::{sync::RwLock, task};
use futures::future::{self, BoxFuture};
use thiserror::Error;

use crate::{device::prelude::*, event::Emitter, ffi::DeviceDatas};

#[ffi_type(namespace = "ffi")]
pub(crate) struct ContextFFI {
    pub(crate) devices: DeviceDatas,
}

#[ffi_type(namespace = "ffi")]
pub(crate) struct Response {
    ok: bool,
    data: *const c_void,
}

unsafe extern "C" {
    fn handle_command(command: *const device_ffi::Command) -> Response;
}

#[allow(private_bounds, private_interfaces)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Device ({0:?} {1}) not found in context")]
    DeviceNotFound(device_ffi::Type, u8),

    #[error("Device ({0:?} {1}) already registered")]
    DeviceAlreadyRegistered(device_ffi::Type, u8),
}

type DeviceFn = Arc<dyn (Fn(&device_ffi::Data) -> BoxFuture<'static, ()>) + Send + Sync>;

pub(crate) struct Context {
    data: Arc<RwLock<HashMap<device_ffi::Device, device_ffi::Data>>>,
    emit_fns: Arc<RwLock<HashMap<device_ffi::Device, DeviceFn>>>,
}

impl Context {
    pub(crate) fn instance() -> &'static Context {
        static INSTANCE: LazyLock<Arc<Context>> = LazyLock::new(|| Arc::new(Context::new()));
        &INSTANCE
    }

    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            emit_fns: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) async fn replace(&self, ffi: DeviceDatas) {
        *self.data.write().await = ffi
            .into_vec()
            .into_iter()
            .map(|data| (data.device, data))
            .collect();

        task::spawn(async move {
            let context = Context::instance();
            let data = context.data.read().await;
            let emit_fns = context.emit_fns.read().await;
            let mut futures = Vec::new();

            for (device, data) in data.iter() {
                let Some(handler) = emit_fns.get(device) else {
                    continue;
                };

                futures.push(handler(data));
            }

            future::join_all(futures).await;
        });
    }

    pub(crate) async fn command<D: Device>(
        &self,
        device: &D,
        command: D::CommandFFI,
    ) -> Result<
        Result<
            *const <D::CommandFFI as device::Command>::Ok,
            *const <D::CommandFFI as device::Command>::Error,
        >,
        Error,
    > {
        if !self.device_exists(device).await {
            return Err(Error::DeviceNotFound(D::TYPE, device.id()));
        }

        let command = device_ffi::Command::new(device, command);
        let res = unsafe { handle_command(&command) };

        if res.ok {
            return Ok(Ok(res.data.cast()));
        }

        Ok(Err(res.data.cast()))
    }

    pub(crate) async fn add_device<D: Device>(&self, device: &Arc<D>) -> Result<(), Error> {
        let device = Arc::clone(device);
        let device_ffi = (&*device).into();

        if self.device_exists(&*device).await {
            return Err(Error::DeviceAlreadyRegistered(D::TYPE, device.id()));
        }

        let callback = Arc::new(move |data: &device_ffi::Data| {
            let device = Arc::clone(&device);
            let ptr = data.data.cast::<D::DataFFI>();
            let deref = unsafe { &*ptr };
            let data = Arc::new(deref.into());
            Box::pin(Emitter::instance().emit_device(device, data)) as BoxFuture<'static, ()>
        });

        let mut devices = self.emit_fns.write().await;
        devices.insert(device_ffi, callback);

        Ok(())
    }

    pub(crate) async fn device_exists<D: Device>(&self, device: &D) -> bool {
        self.emit_fns.read().await.contains_key(&device.into())
    }

    pub(crate) async fn data<D: Device + 'static>(&self, device: &D) -> Option<D::Data> {
        let device = device.into();

        let map = self.data.read().await;
        let data = map.get(&device)?;
        let ptr = data.data.cast::<D::DataFFI>();
        let deref = unsafe { &*ptr };

        Some(deref.into())
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

#[cfg(feature = "build")]
pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(ContextFFI))
        .register(extra_type!(Response))
}
