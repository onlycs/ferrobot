mod ffi;
pub mod prelude;
pub mod spark;

#[cfg(feature = "build")]
use interoptopus::inventory::InventoryBuilder;

pub(crate) trait Device {
    const TYPE: ffi::Type;

    type DataFFI;
    type CommandFFI: Command;

    fn id(&self) -> u8;
}

pub(crate) trait Command {
    type Response;
}

#[cfg(feature = "build")]
pub(crate) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = spark::__ffi_inventory(builder);

    builder
}
