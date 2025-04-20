use std::{collections::HashSet, ffi::c_void, sync::Arc};

use async_std::sync::RwLock;
use thiserror::Error;

use crate::{device::prelude::*, ffi::DeviceDatas};

#[ffi_type(namespace = "ffi")]
#[derive(Debug)]
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

pub struct Context {
    data: Arc<RwLock<Vec<device_ffi::Data>>>,
    devices: Arc<RwLock<HashSet<device_ffi::Device>>>,
}

lazy_static! {
    static ref CONTEXT: Context = Context::new();
}

impl Context {
    pub(crate) fn instance() -> &'static Context {
        &CONTEXT
    }

    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
            devices: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub(crate) async fn replace(&self, ffi: DeviceDatas) {
        *self.data.write().await = ffi.to_vec();
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
            return Ok(Ok(res.data as *const _));
        }

        Ok(Err(res.data as *const _))
    }

    pub(crate) async fn add_device<D: Device>(&self, device: &D) -> Result<(), Error> {
        if self.device_exists(device).await {
            return Err(Error::DeviceAlreadyRegistered(D::TYPE, device.id()));
        }

        let mut devices = self.devices.write().await;
        devices.insert(device.into());

        Ok(())
    }

    pub(crate) async fn device_exists<D: Device>(&self, device: &D) -> bool {
        self.devices.read().await.contains(&device.into())
    }

    pub(crate) async unsafe fn data<'a, D: Device + 'static>(
        &'a self,
        device: &D,
    ) -> Option<&'a D::DataFFI> {
        let device = device.into();
        let data = self.data.read().await;
        let data = data.iter().find(|d| d.device == device)?;
        let data = data.data as *const D::DataFFI;

        unsafe { Some(&*data) }
    }
}

#[cfg(feature = "build")]
pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(ContextFFI))
        .register(extra_type!(Response))
}
