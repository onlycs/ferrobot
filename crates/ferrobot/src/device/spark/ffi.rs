use std::{
    ffi::{CStr, c_char, c_void},
    mem,
};

use interoptopus::ffi::CStrPtr;

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
    type Error = Error;
    type Ok = ();
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
#[derive(Clone, Copy, Debug, PartialEq, derive_more::Display)]
pub enum ErrorType {
    #[display("Tried to create existing motor")]
    MotorExists,
    #[display("Invalid motor configuration")]
    BadConfig,
    #[display("Invalid motor command")]
    BadCommand,
}

#[allow(dead_code)]
#[ffi_type(namespace = "ffi::device::spark")]
#[derive(Debug, thiserror::Error)]
#[error("{kind}: {}", message.as_str().unwrap_or("Unknown error"))]
pub struct Error {
    kind: ErrorType,
    /// Heap-allocated string of which Rust has the responsibility of freeing
    message: CStrPtr<'static>,
}

impl Clone for Error {
    fn clone(&self) -> Self {
        let c_str = self.message.as_c_str().unwrap();
        let message = unsafe {
            let ptr = libc::malloc(c_str.to_bytes().len() + 1) as *mut c_char;
            libc::strcpy(ptr, c_str.as_ptr());
            CStr::from_ptr(ptr)
        };

        Self {
            message: CStrPtr::from_cstr(message),
            kind: self.kind,
        }
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        let Some(c_str) = self.message.as_c_str() else {
            return;
        };

        unsafe {
            let ptr = c_str.as_ptr() as *const c_char;
            if !ptr.is_null() {
                libc::free(ptr as *mut libc::c_void);
            }
        }
    }
}

#[cfg(feature = "build")]
pub(crate) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(CommandType))
        .register(extra_type!(Command))
        .register(extra_type!(Data))
        .register(extra_type!(ErrorType))
        .register(extra_type!(Error))
}
