mod config;
mod ffi;
pub mod prelude;

pub use config::*;
use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};
use prelude::*;
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
        unsafe { Context::instance().read().await.data(self).copied() }
    }

    pub async fn set_position(&self, position: f64) {
        debug!("Setting spark {} position to {}", self.can_id, position);

        let ctx = Context::instance().read().await;
        let command = device_ffi::Command::new(self, spark_ffi::Command::set_position(position));
        ctx.command(command).await;
    }

    pub async fn set_velocity(&self, velocity: f64) {
        debug!("Setting spark {} velocity to {}", self.can_id, velocity);

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
    type Command = spark_ffi::Command;
    type Data = Data;

    const TYPE: super::ffi::DeviceType = super::ffi::DeviceType::SparkMax;

    fn id(&self) -> u8 {
        self.can_id
    }
}

pub(super) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = config::__ffi_inventory(builder);

    builder.register(extra_type!(Data))
}
