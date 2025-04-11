use std::{ffi::c_void, mem};

use super::{MotorType, SparkMaxConfig};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum CommandType {
    SetPosition,
    SetVelocity,
    SetOutput,
    Configure,
    Create,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SparkMaxCommand {
    kind: CommandType,
    data: *const c_void,
}

impl SparkMaxCommand {
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

    pub(crate) fn configure(config: SparkMaxConfig) -> Self {
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

impl Drop for SparkMaxCommand {
    fn drop(&mut self) {
        match self.kind {
            CommandType::SetVelocity | CommandType::SetPosition | CommandType::SetOutput => unsafe {
                mem::drop(Box::from_raw(self.data as *mut f64))
            },
            CommandType::Configure => unsafe {
                mem::drop(Box::from_raw(self.data as *mut SparkMaxConfig))
            },
            CommandType::Create => unsafe { mem::drop(Box::from_raw(self.data as *mut MotorType)) },
        }
    }
}
