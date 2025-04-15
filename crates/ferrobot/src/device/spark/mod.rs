mod config;
pub(crate) mod ffi;

pub use config::*;
pub(crate) use ffi::*;
use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};
use thiserror::Error;

use crate::context::Context;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Motor with ID {0} already exists")]
    AlreadyExists(u8),
}

#[ffi_type(namespace = "ffi::spark")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Data {
    connected: bool,
    position: f64,
    velocity: f64,
    output: f64,
    current: f64,
}

#[derive(Debug)]
pub struct SparkMax {
    can_id: u8,
}

impl SparkMax {
    pub async fn new(can_id: u8, motor_type: MotorType, config: Config) -> Result<Self, Error> {
        let ctx = Context::instance().read().await;
        let ffi_device = super::ffi::Device::spark_max(can_id);

        if ctx.device_exists(&ffi_device).await {
            return Err(Error::AlreadyExists(can_id));
        }

        let create = super::ffi::DeviceCommand {
            device: ffi_device,
            command: ffi::Command::create(motor_type).into_ptr(),
        };

        let configure = super::ffi::DeviceCommand {
            device: ffi_device,
            command: ffi::Command::configure(config).into_ptr(),
        };

        ctx.command(create).await;
        ctx.command(configure).await;

        Ok(Self { can_id })
    }

    pub async fn data(&self) -> Option<Data> {
        unsafe { Context::instance().read().await.data(self).copied() }
    }
}

impl super::Device for SparkMax {
    type Data = Data;

    const KIND: super::ffi::DeviceType = super::ffi::DeviceType::SparkMax;

    fn id(&self) -> u8 {
        self.can_id
    }
}

pub(super) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = config::__ffi_inventory(builder);

    builder.register(extra_type!(Data))
}
