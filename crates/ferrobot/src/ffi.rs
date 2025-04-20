use crate::device::prelude::*;

#[ffi_type(namespace = "ffi")]
#[derive(Debug)]
pub(crate) struct DeviceDatas {
    data: *const device_ffi::Data,
    len: u32,
    cap: u32,
}

impl DeviceDatas {
    pub(crate) fn to_vec(&self) -> Vec<device_ffi::Data> {
        unsafe {
            Vec::from_raw_parts(
                self.data as *mut device_ffi::Data,
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

#[cfg(feature = "build")]
pub(crate) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder.register(extra_type!(DeviceDatas))
}
