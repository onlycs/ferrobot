use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandType {
    SetPosition,
    SetVelocity,
    SetOutput,
    Configure,
    Create,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SparkMaxCommand {
    pub(crate) kind: CommandType,
    pub(crate) data: *const c_void,
}
