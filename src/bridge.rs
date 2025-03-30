#[cxx::bridge]
mod ffi {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum RobotMode {
        Teleoperated,
        Autonomous,
        Test,
        Disabled,
    }

    #[derive(Debug)]
    pub struct ReadContext {
        mode: RobotMode,
    }

    #[derive(Debug)]
    pub enum DevicePort {
        Can,
        Pwm,
    }

    #[derive(Debug)]
    pub enum DeviceType {
        SparkMax,
        SparkFlex,
    }

    #[derive(Debug)]
    pub struct Device {
        port: DevicePort,
        id: u32,
        kind: DeviceType,
    }

    #[derive(Debug)]
    pub struct WriteContext {
        devices: Vec<Device>,
    }

    #[derive(Debug)]
    pub struct Context {
        read: ReadContext,
        write: WriteContext,
    }
}

pub use ffi::*;
