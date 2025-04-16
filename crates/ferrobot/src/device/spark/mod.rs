mod config;
mod ffi;
pub mod prelude;

pub use config::*;
use interoptopus::inventory::InventoryBuilder;
use prelude::*;
use thiserror::Error;
use uom::si::{angle::revolution, angular_velocity::revolution_per_minute as rpm};

use crate::context::Context;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Motor with ID {0} already exists")]
    AlreadyExists(u8),
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
    pub async fn new(can_id: u8, motor_type: MotorType, config: Config) -> Result<Self, Error> {
        let ctx = Context::instance().read().await;
        let this = Self { can_id };

        if ctx.device_exists(&this).await {
            return Err(Error::AlreadyExists(can_id));
        }

        let create = device_ffi::Command::new(&this, spark_ffi::Command::create(motor_type));
        let configure = device_ffi::Command::new(&this, spark_ffi::Command::configure(config));

        info!(
            "Creating SparkMax with ID {} and motor type {:?}",
            can_id, motor_type
        );

        ctx.command(create).await;
        ctx.command(configure).await;

        Ok(Self { can_id })
    }

    pub async fn data(&self) -> Option<Data> {
        let ctx = Context::instance().read().await;
        unsafe { ctx.data(self).copied().map(Data::from) }
    }

    pub async fn set_position(&self, position: Angle) {
        debug!("Setting spark {} position to {:?}", self.can_id, position);

        let position = position.get::<revolution>();
        let ctx = Context::instance().read().await;
        let command = device_ffi::Command::new(self, spark_ffi::Command::set_position(position));
        ctx.command(command).await;
    }

    pub async fn set_velocity(&self, velocity: AngularVelocity) {
        debug!("Setting spark {} velocity to {:?}", self.can_id, velocity);

        let velocity = velocity.get::<rpm>();
        let ctx = Context::instance().read().await;
        let command = device_ffi::Command::new(self, spark_ffi::Command::set_velocity(velocity));
        ctx.command(command).await;
    }

    pub async fn set_output(&self, output: f64) {
        if !(-1.0..=1.0).contains(&output) {
            panic!("`output` must be between -1.0 and 1.0");
        }

        debug!("Setting spark {} output to {}", self.can_id, output);

        let ctx = Context::instance().read().await;
        let command = device_ffi::Command::new(self, spark_ffi::Command::set_output(output));
        ctx.command(command).await;
    }
}

impl super::Device for SparkMax {
    type CommandFFI = spark_ffi::Command;
    type DataFFI = spark_ffi::Data;

    const TYPE: super::ffi::DeviceType = super::ffi::DeviceType::SparkMax;

    fn id(&self) -> u8 {
        self.can_id
    }
}

pub(super) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = config::__ffi_inventory(builder);
    builder
}
