mod config;
pub(crate) mod ffi;

use crate::context::Context;
pub use config::*;
use std::ffi::c_void;

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
    pub fn new(ctx: &Context, can_id: u8, motor_type: MotorType) -> Self {
        let command = super::ffi::DeviceCommand {
            device: super::ffi::Device {
                kind: super::ffi::DeviceType::SparkMax,
                id: can_id,
            },
            command: Box::into_raw(Box::new(ffi::SparkMaxCommand {
                kind: ffi::CommandType::Create,
                data: Box::into_raw(Box::new(motor_type)) as *const c_void,
            })) as *const c_void,
        };

        ctx.command(command);

        Self { can_id }
    }

    pub fn data(&self, ctx: &Context) -> Option<SparkMaxData> {
        unsafe { ctx.data(self).copied() }
    }
}

impl super::Device for SparkMax {
    const KIND: super::ffi::DeviceType = super::ffi::DeviceType::SparkMax;
    type Data = SparkMaxData;

    fn id(&self) -> u8 {
        self.can_id
    }
}
