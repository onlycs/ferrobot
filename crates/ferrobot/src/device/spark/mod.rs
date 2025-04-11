mod config;
pub(crate) mod ffi;

pub use config::*;
use thiserror::Error;

use crate::context::Context;

#[derive(Error, Debug)]
pub enum SparkMaxError {
    #[error("Motor with ID {0} already exists")]
    AlreadyExists(u8),
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SparkMaxData {
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
    pub async fn new(
        can_id: u8,
        motor_type: MotorType,
        config: SparkMaxConfig,
    ) -> Result<Self, SparkMaxError> {
        let ctx = Context::instance().read().await;
        let ffi_device = super::ffi::Device::spark_max(can_id);

        if ctx.device_exists(&ffi_device).await {
            return Err(SparkMaxError::AlreadyExists(can_id));
        }

        let create = super::ffi::DeviceCommand {
            device: ffi_device,
            command: ffi::SparkMaxCommand::create(motor_type).into_ptr(),
        };

        let configure = super::ffi::DeviceCommand {
            device: ffi_device,
            command: ffi::SparkMaxCommand::configure(config).into_ptr(),
        };

        ctx.command(create).await;
        ctx.command(configure).await;

        Ok(Self { can_id })
    }

    pub async fn data(&self) -> Option<SparkMaxData> {
        unsafe { Context::instance().read().await.data(self).copied() }
    }
}

impl super::Device for SparkMax {
    type Data = SparkMaxData;

    const KIND: super::ffi::DeviceType = super::ffi::DeviceType::SparkMax;

    fn id(&self) -> u8 {
        self.can_id
    }
}
