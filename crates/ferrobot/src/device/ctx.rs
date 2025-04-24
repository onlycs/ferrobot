use std::{collections::HashMap, sync::LazyLock};

use async_std::{sync::RwLock, task};
use futures::future::{self, BoxFuture};
use thiserror::Error;

use super::prelude::*;
use crate::{event::Emitter, ffi::DeviceDatas};

unsafe extern "C" {
    fn handle_command(command: *const device_ffi::Command) -> ferrobot_ffi::Response;
}

#[allow(private_bounds, private_interfaces)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Device ({0:?} {1}) not found in context")]
    DeviceNotFound(device_ffi::Type, u8),

    #[error("Device ({0:?} {1}) already registered")]
    DeviceAlreadyRegistered(device_ffi::Type, u8),
}

type Emit = Arc<dyn (Fn(&device_ffi::Data) -> BoxFuture<'static, ()>) + Send + Sync>;

pub(crate) struct DeviceContext {
    data: Arc<RwLock<HashMap<device_ffi::Device, device_ffi::Data>>>,
    emitters: Arc<RwLock<HashMap<device_ffi::Device, Emit>>>,
}

impl DeviceContext {
    pub(crate) fn instance() -> &'static DeviceContext {
        static INSTANCE: LazyLock<Arc<DeviceContext>> = LazyLock::new(DeviceContext::new);
        &INSTANCE
    }

    fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            emitters: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub(crate) async fn replace(&self, ffi: DeviceDatas) {
        *self.data.write().await = ffi
            .into_vec()
            .into_iter()
            .map(|data| (data.device, data))
            .collect();

        task::spawn(async move {
            let ctx = DeviceContext::instance();
            let data = ctx.data.read().await;
            let emitters = ctx.emitters.read().await;
            let mut futures = Vec::new();

            for (device, data) in data.iter() {
                let Some(handler) = emitters.get(device) else {
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

        let mut devices = self.emitters.write().await;
        devices.insert(device_ffi, callback);

        Ok(())
    }

    pub(crate) async fn device_exists<D: Device>(&self, device: &D) -> bool {
        self.emitters.read().await.contains_key(&device.into())
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

unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}
