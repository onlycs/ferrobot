use interoptopus::inventory::InventoryBuilder;

mod ffi;
pub mod prelude;
pub mod spark;

pub(crate) trait Device {
    const TYPE: ffi::DeviceType;

    type DataFFI;
    type CommandFFI;

    fn id(&self) -> u8;
}

pub(super) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = spark::__ffi_inventory(builder);

    builder
}
