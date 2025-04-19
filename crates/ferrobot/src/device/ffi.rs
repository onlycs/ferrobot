use std::ffi::c_void;

#[cfg(feature = "build")]
use interoptopus::{extra_type, ffi_function, ffi_type, function, inventory::InventoryBuilder};

use super::spark::SparkMax;

#[cfg_attr(feature = "build", ffi_type(namespace = "ffi::device"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Type {
    SparkMax,
    NavX,
    XboxController,
}

#[cfg_attr(feature = "build", ffi_type(namespace = "ffi::device"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Device {
    kind: Type,
    id: u8,
}

impl<D: super::Device> From<&D> for Device {
    fn from(device: &D) -> Self {
        Self {
            kind: D::TYPE,
            id: device.id(),
        }
    }
}

#[cfg_attr(feature = "build", ffi_type(namespace = "ffi::device"))]
#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) device: Device,
    pub(crate) data: *const c_void,
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                Type::SparkMax => drop(Box::from_raw(
                    self.data as *mut <SparkMax as super::Device>::DataFFI,
                )),
                Type::NavX => unimplemented!(), // TODO: NavX
                Type::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for Data {}
unsafe impl Sync for Data {}

#[cfg_attr(feature = "build", ffi_type(namespace = "ffi::device"))]
#[derive(Debug)]
pub(crate) struct Command {
    pub(crate) device: Device,
    pub(crate) command: *const c_void,
}

impl Command {
    pub fn new<D: super::Device>(device: &D, command: D::CommandFFI) -> Self {
        Self {
            device: device.into(),
            command: Box::into_raw(Box::new(command)) as *const c_void,
        }
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                Type::SparkMax => drop(Box::from_raw(
                    self.command as *mut <SparkMax as super::Device>::CommandFFI,
                )),
                Type::NavX => unimplemented!(), // TODO: NavX
                Type::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

#[cfg_attr(feature = "build", ffi_function(namespace = "ffi::device"))]
fn command_free(command: Command) {
    drop(command);
}

// Pointer is read-only
unsafe impl Send for Command {}
unsafe impl Sync for Command {}

#[cfg(feature = "build")]
pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(Type))
        .register(extra_type!(Data))
        .register(extra_type!(Command))
        .register(function!(command_free))
}
