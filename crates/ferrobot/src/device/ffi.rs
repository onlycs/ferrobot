use std::ffi::c_void;

use interoptopus::{extra_type, ffi_type, inventory::InventoryBuilder};

use super::spark;

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
    pub(crate) kind: DeviceType,
    pub(crate) id: u8,
}

impl Device {
    pub(crate) fn spark_max(can_id: u8) -> Self {
        Self {
            kind: DeviceType::SparkMax,
            id: can_id,
        }
    }

    pub(crate) fn xbox_controller(port: u8) -> Self {
        Self {
            kind: DeviceType::XboxController,
            id: port,
        }
    }
}

#[ffi_type(namespace = "ffi::device")]
#[derive(Debug)]
pub(crate) struct DeviceData {
    pub(crate) device: Device,
    pub(crate) data: *const c_void,
}

impl Drop for DeviceData {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                DeviceType::SparkMax => drop(Box::from_raw(self.data as *mut spark::Data)),
                DeviceType::NavX => unimplemented!(), // TODO: NavX
                DeviceType::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for DeviceData {}
unsafe impl Sync for DeviceData {}

#[ffi_type(namespace = "ffi::device")]
#[derive(Debug)]
pub(crate) struct DeviceCommand {
    pub(crate) device: Device,
    pub(crate) command: *const c_void,
}

impl Drop for DeviceCommand {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                DeviceType::SparkMax => drop(Box::from_raw(self.command as *mut spark::Command)),
                DeviceType::NavX => unimplemented!(), // TODO: NavX
                DeviceType::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for DeviceCommand {}
unsafe impl Sync for DeviceCommand {}

pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(DeviceType))
        .register(extra_type!(DeviceData))
        .register(extra_type!(DeviceCommand))
}
