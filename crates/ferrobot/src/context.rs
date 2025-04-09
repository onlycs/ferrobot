use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    device::{
        self, Device,
        ffi::{DeviceCommand, DeviceData},
    },
    ffi::DeviceDatas,
};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct ContextFFI {
    pub(crate) devices: DeviceDatas,
}

#[derive(Debug)]
pub struct Context {
    devices: Vec<DeviceData>,
    queue: Arc<Mutex<VecDeque<DeviceCommand>>>,
}

impl Context {
    pub(crate) fn new(ffi: DeviceDatas, queue: Arc<Mutex<VecDeque<DeviceCommand>>>) -> Self {
        Self {
            devices: ffi.to_vec(),
            queue,
        }
    }

    pub(crate) fn command(&self, command: device::ffi::DeviceCommand) {
        let mut queue = match self.queue.lock() {
            Ok(queue) => queue,
            Err(poisoned) => poisoned.into_inner(),
        };

        queue.push_back(command);
    }

    pub(crate) unsafe fn data<D: Device>(&self, device: &D) -> Option<&D::Data> {
        let device = device::ffi::Device {
            kind: D::KIND,
            id: device.id(),
        };

        let data = self.devices.iter().find(|d| d.device == device)?;
        let data = unsafe { &*(data.data as *const D::Data) };

        Some(data)
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}
