use std::mem;

use interoptopus::{extra_type, ffi_type, ffi_function, function, inventory::InventoryBuilder};

use crate::device::ffi::{DeviceCommand, DeviceData};

#[ffi_type(namespace = "ffi")]
#[derive(Debug)]
pub(crate) struct DeviceCommands {
    data: *const DeviceCommand,
    len: u32,
    cap: u32,
}

impl DeviceCommands {
    pub(crate) fn new(from: Vec<DeviceCommand>) -> Self {
        let ffi = DeviceCommands {
            data: from.as_ptr(),
            cap: from.capacity() as u32,
            len: from.len() as u32,
        };

        mem::forget(from);

        ffi
    }
}

impl Drop for DeviceCommands {
    fn drop(&mut self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len as usize, self.cap as usize);
        }
    }
}

unsafe impl Send for DeviceCommands {}
unsafe impl Sync for DeviceCommands {}

#[ffi_type(namespace = "ffi")]
#[derive(Debug)]
pub(crate) struct DeviceDatas {
    data: *const DeviceData,
    len: u32,
    cap: u32,
}

impl DeviceDatas {
    pub(crate) fn to_vec(&self) -> Vec<DeviceData> {
        unsafe {
            Vec::from_raw_parts(
                self.data as *mut DeviceData,
                self.len as usize,
                self.cap as usize,
            )
        }
    }

    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn device_datas_free(self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len as usize, self.cap as usize);
        }
    }
}

impl Drop for DeviceDatas {
    fn drop(&mut self) {
        unsafe {
            Vec::<u8>::from_raw_parts(self.data as *mut u8, self.len as usize, self.cap as usize);
        }
    }
}

unsafe impl Send for DeviceDatas {}
unsafe impl Sync for DeviceDatas {}


#[ffi_function(namespace = "ffi")]
pub(crate) unsafe fn device_commands_free(commands: DeviceCommands) {
    unsafe {
        Vec::<u8>::from_raw_parts(commands.data as *mut u8, commands.len as usize, commands.cap as usize);
    }
}

pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(DeviceCommands))
        .register(extra_type!(DeviceDatas))
        .register(function!(device_commands_free))
}



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
