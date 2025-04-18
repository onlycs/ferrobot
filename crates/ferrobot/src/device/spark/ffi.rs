use std::{ffi::c_void, mem};

use interoptopus::ffi_type;

use super::prelude::*;

#[ffi_type(namespace = "ffi::device::spark")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum CommandType {
    SetPosition,
    SetVelocity,
    SetOutput,
    Create,
}

#[ffi_type(namespace = "ffi::device::spark")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Command {
    kind: CommandType,
    data: *const c_void,
}

impl Command {
    pub(crate) fn set_position(position: f64) -> Self {
        Self {
            kind: CommandType::SetPosition,
            data: Box::into_raw(Box::new(position)) as *const c_void,
        }
    }

    pub(crate) fn set_velocity(velocity: f64) -> Self {
        Self {
            kind: CommandType::SetVelocity,
            data: Box::into_raw(Box::new(velocity)) as *const c_void,
        }
    }

    pub(crate) fn set_output(output: f64) -> Self {
        Self {
            kind: CommandType::SetOutput,
            data: Box::into_raw(Box::new(output)) as *const c_void,
        }
    }

    pub(crate) fn create(config: spark::SparkMaxConfig) -> Self {
        Self {
            kind: CommandType::Create,
            data: Box::into_raw(Box::new(config)) as *const c_void,
        }
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        match self.kind {
            CommandType::SetVelocity | CommandType::SetPosition | CommandType::SetOutput => unsafe {
                mem::drop(Box::from_raw(self.data as *mut f64))
            },
            CommandType::Create => unsafe {
                mem::drop(Box::from_raw(self.data as *mut spark::SparkMaxConfig))
            },
        }
    }
}

impl device::Command for Command {
    type Response = Response;
}

#[ffi_type(namespace = "ffi::device::spark")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Data {
    pub(crate) connected: bool,
    pub(crate) output: f64,
    pub(crate) position: f64,
    pub(crate) velocity: f64,
    pub(crate) current: f64,
}

#[allow(dead_code)]
#[ffi_type(namespace = "ffi::device::spark")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Response {
    Ok,
    MotorExists,
    BadConfig,
    BadCommand,
}

#[cfg(feature = "build")]
pub(crate) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(CommandType))
        .register(extra_type!(Command))
        .register(extra_type!(Data))
        .register(extra_type!(Response))
}
