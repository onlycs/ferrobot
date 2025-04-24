pub(crate) mod device_ffi {
    pub(crate) use crate::device::ffi::*;
}

pub(crate) use super::ctx::{self as device_ctx, DeviceContext};
pub use crate::{
    device::{self, Device, spark},
    prelude::*,
};
