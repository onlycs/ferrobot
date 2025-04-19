mod config;
mod ffi;
pub mod prelude;

use std::{backtrace::Backtrace, panic::Location};

pub use config::*;
use prelude::*;
use thiserror::Error;
use uom::si::{angle::revolution, angular_velocity::revolution_per_minute as rpm};

use crate::context::Context;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Motor (ID {0}) already exists")]
    AlreadyExists(u8),
    #[error("Invalid configuration for motor (ID {0})")]
    InvalidConfig(u8),
    #[error("Bad `set_output` command: Expected -1.0 to 1.0, got {0}")]
    InvalidOutput(f64),
    #[error("At {location}: Context error: {source:?}")]
    ContextError {
        #[from]
        source: context::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

impl ffi::Response {
    pub fn to_result(self, can_id: u8) -> Result<(), Error> {
        match self {
            ffi::Response::Ok => Ok(()),
            ffi::Response::MotorExists => Err(Error::AlreadyExists(can_id)),
            ffi::Response::BadConfig => Err(Error::InvalidConfig(can_id)),
            ffi::Response::BadCommand => {
                panic!("An invalid command was sent to the motor. This is a bug; please report it.")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Data {
    pub connected: bool,
    pub output: f64,
    pub position: Angle,
    pub velocity: AngularVelocity,
    pub current: ElectricCurrent,
}

impl From<spark_ffi::Data> for Data {
    fn from(value: spark_ffi::Data) -> Self {
        Self {
            connected: value.connected,
            output: value.output,
            position: Angle::new::<revolution>(value.position),
            velocity: AngularVelocity::new::<rpm>(value.velocity),
            current: ElectricCurrent::new::<amp>(value.current),
        }
    }
}

#[derive(Debug)]
pub struct SparkMax {
    can_id: u8,
}

impl SparkMax {
    pub async fn new(can_id: u8, config: SparkMaxConfig) -> Result<Self, Error> {
        let ctx = Context::instance().read().await;
        let this = Self { can_id };
        let command = spark_ffi::Command::create(config);

        ctx.add_device(&this).await?;
        ctx.command(&this, command).await?.to_result(can_id)?;

        Ok(this)
    }

    pub async fn data(&self) -> Option<Data> {
        let ctx = Context::instance().read().await;
        unsafe { ctx.data(self).copied().map(Data::from) }
    }

    pub async fn set_position(&self, position: Angle) -> Result<(), Error> {
        debug!("Setting spark {} position to {:?}", self.can_id, position);

        let position = position.get::<revolution>();
        let ctx = Context::instance().read().await;
        let command = spark_ffi::Command::set_position(position);

        ctx.command(self, command).await?.to_result(self.can_id)
    }

    pub async fn set_velocity(&self, velocity: AngularVelocity) -> Result<(), Error> {
        debug!("Setting spark {} velocity to {:?}", self.can_id, velocity);

        let velocity = velocity.get::<rpm>();
        let ctx = Context::instance().read().await;
        let command = spark_ffi::Command::set_velocity(velocity);

        ctx.command(self, command).await?.to_result(self.can_id)
    }

    pub async fn set_output(&self, output: f64) -> Result<(), Error> {
        if !(-1.0..=1.0).contains(&output) {
            return Err(Error::InvalidOutput(output));
        }

        debug!("Setting spark {} output to {}", self.can_id, output);

        let ctx = Context::instance().read().await;
        let command = spark_ffi::Command::set_output(output);

        ctx.command(self, command).await?.to_result(self.can_id)
    }
}

impl device::DeviceFFI for SparkMax {
    type CommandFFI = spark_ffi::Command;
    type DataFFI = spark_ffi::Data;

    const TYPE: device_ffi::Type = device_ffi::Type::SparkMax;
}

impl device::Device for SparkMax {
    type Data = Data;

    fn id(&self) -> u8 {
        self.can_id
    }
}

#[cfg(feature = "build")]
pub(super) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = config::__ffi_inventory(builder);
    builder
}
