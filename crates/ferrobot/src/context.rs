use std::{collections::HashSet, ffi::c_void, sync::Arc};

use async_std::sync::{RwLock, RwLockReadGuard};
use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};

use crate::{
    device::prelude::*,
    ffi::{CBox, DeviceDatas},
};

#[ffi_type(namespace = "ffi")]
#[derive(Debug)]
pub(crate) struct ContextFFI {
    pub(crate) devices: DeviceDatas,
}

#[derive(Debug)]
pub(crate) struct ContextInner {
    data: Vec<device_ffi::Data>,
    devices: RwLock<HashSet<device_ffi::Device>>,
}

impl ContextInner {
    pub(crate) async fn command<D: device::Device>(
        &self,
        device: &D,
        command: D::CommandFFI,
    ) -> CBox<<D::CommandFFI as device::Command>::Response> {
        self.devices.write().await.insert(device.into());

        let command = device_ffi::Command::new(device, command);
        let res = unsafe { handle_command(command) };

        unsafe { CBox::new(res) }
    }

    pub(crate) async fn device_exists<D: Device>(&self, device: &D) -> bool {
        self.devices.read().await.contains(&device.into())
    }

    pub(crate) unsafe fn data<D: Device>(&self, device: &D) -> Option<&D::DataFFI> {
        let device = device.into();
        let data = self.data.iter().find(|d| d.device == device)?;
        let data = unsafe { &*(data.data as *const D::DataFFI) };

        Some(data)
    }
}

unsafe extern "C" {
    fn handle_command(command: device_ffi::Command) -> *mut c_void;
}

pub struct Context(Arc<RwLock<ContextInner>>);

static mut CONTEXT: Option<Context> = None;

impl Context {
    #[allow(static_mut_refs)]
    pub(crate) fn instance() -> &'static Context {
        unsafe {
            if CONTEXT.is_none() {
                CONTEXT = Some(Self::new(ContextInner {
                    data: vec![],
                    devices: RwLock::new(HashSet::new()),
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

#[cfg(feature = "build")]
pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder.register(extra_type!(ContextFFI))
}
