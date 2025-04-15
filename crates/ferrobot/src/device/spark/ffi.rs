use std::{ffi::c_void, mem};

use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};

use super::{Config, MotorType};

#[ffi_type(namespace = "ffi::spark")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum CommandType {
    SetPosition,
    SetVelocity,
    SetOutput,
    Configure,
    Create,
}

#[ffi_type(namespace = "ffi::spark")]
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

    pub(crate) fn configure(config: Config) -> Self {
        Self {
            kind: CommandType::Configure,
            data: Box::into_raw(Box::new(config)) as *const c_void,
        }
    }

    pub(crate) fn create(motor_type: MotorType) -> Self {
        Self {
            kind: CommandType::Create,
            data: Box::into_raw(Box::new(motor_type)) as *const c_void,
        }
    }

    pub(crate) fn into_ptr(self) -> *const c_void {
        Box::into_raw(Box::new(self)) as *const c_void
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        match self.kind {
            CommandType::SetVelocity | CommandType::SetPosition | CommandType::SetOutput => unsafe {
                mem::drop(Box::from_raw(self.data as *mut f64))
            },
            CommandType::Configure => unsafe { mem::drop(Box::from_raw(self.data as *mut Config)) },
            CommandType::Create => unsafe { mem::drop(Box::from_raw(self.data as *mut MotorType)) },
        }
    }
}

pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(CommandType))
        .register(extra_type!(Command))
}
