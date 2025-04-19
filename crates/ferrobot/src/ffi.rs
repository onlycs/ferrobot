use std::{
    ffi::c_void,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use interoptopus::ffi_type;

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

pub(crate) struct CBox<T> {
    ptr: *mut c_void,
    _ph: PhantomData<T>,
}

impl<T> CBox<T> {
    pub(crate) unsafe fn new(ptr: *mut c_void) -> Self {
        Self {
            ptr,
            _ph: PhantomData,
        }
    }
}

impl<T> Deref for CBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr as *const T) }
    }
}

impl<T> DerefMut for CBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}

impl<T> Drop for CBox<T> {
    fn drop(&mut self) {
        // it was created in libc, and only there can it be destroyed
        unsafe { libc::free(self.ptr) }
    }
}

unsafe impl<T: Send> Send for CBox<T> {}
unsafe impl<T: Sync> Sync for CBox<T> {}

#[cfg(feature = "build")]
pub(super) mod build {
    use interoptopus::{extra_type, inventory::InventoryBuilder};

    use super::*;

    pub(crate) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
        builder.register(extra_type!(DeviceDatas))
    }
}
