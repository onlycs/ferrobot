pub(crate) mod ffi;
pub mod spark;

pub(crate) trait Device {
    const KIND: ffi::DeviceType;
    type Data;

    fn id(&self) -> u8;
}
