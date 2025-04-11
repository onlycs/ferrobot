use std::ffi::c_void;

use super::spark::{SparkMaxData, ffi::SparkMaxCommand};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum DeviceType {
    SparkMax,
    NavX,
    XboxController,
}

#[repr(C)]
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

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DeviceData {
    pub(crate) device: Device,
    pub(crate) data: *const c_void,
}

impl Drop for DeviceData {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                DeviceType::SparkMax => drop(Box::from_raw(self.data as *mut SparkMaxData)),
                DeviceType::NavX => unimplemented!(), // TODO: NavX
                DeviceType::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for DeviceData {}
unsafe impl Sync for DeviceData {}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DeviceCommand {
    pub(crate) device: Device,
    pub(crate) command: *const c_void,
}

impl Drop for DeviceCommand {
    fn drop(&mut self) {
        unsafe {
            match self.device.kind {
                DeviceType::SparkMax => drop(Box::from_raw(self.command as *mut SparkMaxCommand)),
                DeviceType::NavX => unimplemented!(), // TODO: NavX
                DeviceType::XboxController => unimplemented!(), // TODO: XboxController
            }
        }
    }
}

// Pointer is read-only
unsafe impl Send for DeviceCommand {}
unsafe impl Sync for DeviceCommand {}
