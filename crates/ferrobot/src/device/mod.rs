use interoptopus::inventory::InventoryBuilder;

pub(crate) mod ffi;
pub mod spark;

pub(crate) trait Device {
    const KIND: ffi::DeviceType;
    type Data;

    fn id(&self) -> u8;

    fn as_ffi(&self) -> ffi::Device {
        ffi::Device {
            kind: Self::KIND,
            id: self.id(),
        }
    }
}

pub(super) fn __ffi_inventory(mut builder: InventoryBuilder) -> InventoryBuilder {
    builder = ffi::__ffi_inventory(builder);
    builder = spark::__ffi_inventory(builder);

    builder
}
