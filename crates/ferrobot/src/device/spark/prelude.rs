pub use crate::device::prelude::*;

pub(crate) mod spark_ffi {
    pub(crate) use crate::device::spark::ffi::*;
}

pub use crate::device::spark::{self, SparkMax};
