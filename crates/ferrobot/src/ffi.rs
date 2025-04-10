use std::mem;

use crate::device::ffi::{DeviceCommand, DeviceData};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DeviceCommands {
    data: *const DeviceCommand,
    len: usize,
    cap: usize,
}

impl DeviceCommands {
    pub(crate) fn new(from: Vec<DeviceCommand>) -> Self {
        let ffi = DeviceCommands {
            data: from.as_ptr(),
            cap: from.capacity(),
            len: from.len(),
        };

        mem::forget(from);

        ffi
    }

    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn dealloc_commands(self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len, self.cap);
        }
    }
}

impl Drop for DeviceCommands {
    fn drop(&mut self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len, self.cap);
        }
    }
}

unsafe impl Send for DeviceCommands {}
unsafe impl Sync for DeviceCommands {}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DeviceDatas {
    data: *const DeviceData,
    len: usize,
    cap: usize,
}

impl DeviceDatas {
    pub(crate) fn to_vec(&self) -> Vec<DeviceData> {
        unsafe { Vec::from_raw_parts(self.data as *mut DeviceData, self.len, self.cap) }
    }

    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn dealloc_datas(self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len, self.cap);
        }
    }
}

impl Drop for DeviceDatas {
    fn drop(&mut self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len, self.cap);
        }
    }
}

unsafe impl Send for DeviceDatas {}
unsafe impl Sync for DeviceDatas {}

// #[allow(clippy::module_inception)]
// #[cxx::bridge]
// mod ffi {
//     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
//     pub enum RobotMode {
//         Teleoperated,
//         Autonomous,
//         Test,
//         Disabled,
//     }

//     #[derive(Clone, Copy, Debug, PartialEq)]
//     pub struct GyroData {
//         connected: bool,
//         heading: f64,
//         rate: f64,
//     }

//     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//     #[repr(u8)]
//     pub enum NavXConnection {
//         SPI = 0,
//         UART = 1,
//         USB1 = 2,
//         USB2 = 3,
//         I2C = 4,
//     }
// }
