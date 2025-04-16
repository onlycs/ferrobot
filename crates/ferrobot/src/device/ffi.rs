use std::ffi::c_void;

use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};

use super::spark::SparkMax;

#[ffi_type(namespace = "ffi::device")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum DeviceType {
    SparkMax,
    NavX,
    XboxController,
}

#[ffi_type(namespace = "ffi::device")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Device {
    kind: DeviceType,
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

#[ffi_type(namespace = "ffi::device")]
#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) device: Device,
    pub(crate) data: *const c_void,
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                DeviceType::SparkMax => drop(Box::from_raw(
                    self.data as *mut <SparkMax as super::Device>::DataFFI,
                )),
                DeviceType::NavX => unimplemented!(), // TODO: NavX
                DeviceType::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for Data {}
unsafe impl Sync for Data {}

#[ffi_type(namespace = "ffi::device")]
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
                DeviceType::SparkMax => drop(Box::from_raw(
                    self.command as *mut <SparkMax as super::Device>::CommandFFI,
                )),
                DeviceType::NavX => unimplemented!(), // TODO: NavX
                DeviceType::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for Command {}
unsafe impl Sync for Command {}

pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(DeviceType))
        .register(extra_type!(Data))
        .register(extra_type!(Command))
}
