use crate::{start_thread, supply};

#[allow(clippy::module_inception)]
#[cxx::bridge]
mod ffi {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum RobotMode {
        Teleoperated,
        Autonomous,
        Test,
        Disabled,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum DeviceType {
        SparkMax,
        NavX,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct SparkMaxId {
        can_id: u32,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u8)]
    enum NavXConnection {
        SPI = 0,
        UART = 1,
        Usb1 = 2,
        Usb2 = 3,
        I2C = 4,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct SparkMaxData {
        connected: bool,
        position: f64,
        velocity: f64,
        output: f64,
        current: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct GyroData {
        connected: bool,
        heading: f64,
        rate: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Device {
        kind: DeviceType,
        id: u8,
    }

    #[derive(Clone, Copy, Debug)]
    struct DeviceData {
        device: Device,
        data: *mut u8,
    }

    #[derive(Debug, Default)]
    pub struct Context {
        mode: RobotMode,
        devices: Vec<DeviceData>,
    }

    extern "Rust" {
        fn start_thread();
        fn supply(ctx: Context);
    }
}

impl Default for RobotMode {
    fn default() -> Self {
        RobotMode::Disabled
    }
}

pub use ffi::*;
