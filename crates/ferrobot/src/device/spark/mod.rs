mod config;
pub(crate) mod ffi;

use std::ffi::c_void;

pub use config::*;

use crate::context::Context;

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
    pub async fn new(can_id: u8, motor_type: MotorType) -> Self {
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

        Context::instance().read().await.command(command);

        Self { can_id }
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
