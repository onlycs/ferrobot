use std::{ffi::c_void, slice};

use crate::device::prelude::*;

#[ffi_type(namespace = "ffi")]
pub(crate) struct DeviceDatas {
    data: *const device_ffi::Data,
    len: usize,
}

impl DeviceDatas {
    #[allow(clippy::mut_from_ref)]
    fn as_slice(&self) -> &mut [device_ffi::Data] {
        unsafe { slice::from_raw_parts_mut(self.data.cast_mut(), self.len) }
    }

    pub(crate) fn into_vec(self) -> Vec<device_ffi::Data> {
        self.as_slice()
            .iter_mut()
            .map(device_ffi::Data::take)
            .collect()
    }
}

impl Drop for DeviceDatas {
    fn drop(&mut self) {
        unsafe { libc::free(self.data as *mut _) }
    }
}

unsafe impl Send for DeviceDatas {}
unsafe impl Sync for DeviceDatas {}

#[ffi_type(namespace = "ffi")]
pub(crate) struct FFIData {
    pub(crate) devices: DeviceDatas,
}

#[ffi_type(namespace = "ffi")]
pub(crate) struct Response {
    pub(crate) ok: bool,
    pub(crate) data: *const c_void,
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}

#[cfg(feature = "build")]
pub(crate) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(DeviceDatas))
        .register(extra_type!(FFIData))
        .register(extra_type!(Response))
}
