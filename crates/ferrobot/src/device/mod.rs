mod ffi;
pub mod prelude;
pub mod spark;

use prelude::*;

pub(crate) trait DeviceFFI {
    const TYPE: device_ffi::Type;

    type DataFFI;
    type CommandFFI: Command;
}

pub(crate) trait Command {
    type Response;
}

#[allow(private_bounds)]
pub trait Device: DeviceFFI {
    type Data: From<Self::DataFFI>;

    fn id(&self) -> u8;
}

#[cfg(feature = "build")]
pub(crate) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = spark::__ffi_inventory(builder);

    builder
}
