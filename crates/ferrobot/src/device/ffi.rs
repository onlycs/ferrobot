use std::ffi::c_void;

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

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DeviceData {
    pub(crate) device: Device,
    pub(crate) data: *const c_void,
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

// Pointer is read-only
unsafe impl Send for DeviceCommand {}
unsafe impl Sync for DeviceCommand {}
