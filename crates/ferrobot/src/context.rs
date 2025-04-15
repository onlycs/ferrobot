use std::{
    collections::{HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use async_std::sync::{RwLock, RwLockReadGuard};
use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};

use crate::{
    device::{
        self, Device,
        ffi::{DeviceCommand, DeviceData},
    },
    ffi::DeviceDatas,
};

#[ffi_type(namespace = "ffi")]
#[derive(Debug)]
pub(crate) struct ContextFFI {
    pub(crate) devices: DeviceDatas,
}

#[derive(Debug)]
pub(crate) struct ContextInner {
    data: Vec<DeviceData>,
    devices: RwLock<HashSet<device::ffi::Device>>,
    queue: Arc<Mutex<VecDeque<DeviceCommand>>>,
}

impl ContextInner {
    pub(crate) async fn command(&self, command: device::ffi::DeviceCommand) {
        self.devices.write().await.insert(command.device);

        let mut queue = match self.queue.lock() {
            Ok(queue) => queue,
            Err(poisoned) => poisoned.into_inner(),
        };

        queue.push_back(command);
    }

    pub(crate) async fn device_exists(&self, device: &device::ffi::Device) -> bool {
        self.devices.read().await.contains(device)
    }

    pub(crate) unsafe fn data<D: Device>(&self, device: &D) -> Option<&D::Data> {
        let device = device.as_ffi();
        let data = self.data.iter().find(|d| d.device == device)?;
        let data = unsafe { &*(data.data as *const D::Data) };

        Some(data)
    }
}

pub struct Context(Arc<RwLock<ContextInner>>);

static mut CONTEXT: Option<Context> = None;

impl Context {
    #[allow(static_mut_refs)]
    pub(crate) fn instance() -> &'static Context {
        unsafe {
            if CONTEXT.is_none() {
                let queue = crate::QUEUE.as_ref().unwrap();

                CONTEXT = Some(Self::new(ContextInner {
                    data: vec![],
                    devices: RwLock::new(HashSet::new()),
                    queue: Arc::clone(queue),
                }));
            }

            CONTEXT.as_ref().unwrap()
        }
    }

    fn new(inner: ContextInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }

    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, ContextInner> {
        self.0.read().await
    }

    pub(crate) async fn replace(&self, ffi: DeviceDatas) {
        let mut context = self.0.write().await;
        context.data = ffi.to_vec();
    }
}

pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder.register(extra_type!(ContextFFI))
}
