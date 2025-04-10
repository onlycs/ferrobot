use std::collections::VecDeque;

use async_std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::{
    device::{
        self, Device,
        ffi::{DeviceCommand, DeviceData},
    },
    ffi::DeviceDatas,
    prelude::*,
};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct ContextFFI {
    pub(crate) devices: DeviceDatas,
}

#[derive(Debug)]
pub(crate) struct ContextInner {
    devices: Vec<DeviceData>,
    queue: Arc<Mutex<VecDeque<DeviceCommand>>>,
}

impl ContextInner {
    pub(crate) fn command(&self, command: device::ffi::DeviceCommand) {
        let mut queue = match self.queue.lock() {
            Ok(queue) => queue,
            Err(poisoned) => poisoned.into_inner(),
        };

        queue.push_back(command);
    }

    pub(crate) unsafe fn data<D: Device>(&self, device: &D) -> Option<&D::Data> {
        let device = device::ffi::Device {
            kind: D::KIND,
            id: device.id(),
        };

        let data = self.devices.iter().find(|d| d.device == device)?;
        let data = unsafe { &*(data.data as *const D::Data) };

        Some(data)
    }
}

pub struct Context(RwLock<ContextInner>);

static mut CONTEXT: Option<Context> = None;

impl Context {
    #[allow(static_mut_refs)]
    pub(crate) fn instance() -> &'static Context {
        unsafe {
            if CONTEXT.is_none() {
                let queue = crate::QUEUE.as_ref().unwrap();

                CONTEXT = Some(Context(RwLock::new(ContextInner {
                    devices: vec![],
                    queue: Arc::clone(queue),
                })));
            }

            CONTEXT.as_ref().unwrap()
        }
    }

    pub(crate) async fn read(&self) -> RwLockReadGuard<'_, ContextInner> {
        self.0.read().await
    }

    pub(crate) async fn write(&self) -> RwLockWriteGuard<'_, ContextInner> {
        self.0.write().await
    }

    pub(crate) async fn replace(&self, ffi: DeviceDatas) {
        let mut context = self.write().await;
        context.devices = ffi.to_vec();
    }
}
