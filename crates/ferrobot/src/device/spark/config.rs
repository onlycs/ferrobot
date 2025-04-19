use std::time::Duration;

use interoptopus::ffi_type;
use typed_builder::TypedBuilder;

use super::{
    SparkMax,
    prelude::{device::Device, *},
};

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, TypedBuilder)]
pub struct AbsoluteEncoderConfig {
    /// Set the phase of the encoder so that it is in phase with the motor
    /// itself.
    #[builder(default = false)]
    pub inverted: bool,

    /// Set the conversion factor for the position of the absolute encoder.
    /// The native unit is rotations and values will be multiplied by this
    /// conversion factor
    ///
    /// This is NOT RECOMMENDED, you will break uom outputs if you use this
    #[builder(default = 1.0)]
    pub position_factor: f64,

    /// Set the conversion factor for the velocity of the absolute encoder.
    /// The native unit is rotations per minute and values will be multiplied by
    /// this conversion factor
    ///
    /// This is NOT RECOMMENDED, you will break uom outputs if you use this
    #[builder(default = 1.0)]
    pub velocity_factor: f64,

    /// Set the zero offset of the absolute encoder, i.e. the position that is
    /// reported as zero.
    ///
    /// The zero offset is specified as the reported position of the encoder in
    /// the desired zero position as if the zero offset was set to 0,
    /// the position conversion factor was set to 1, and inverted was set to
    /// false.
    #[builder(default = 0.0)]
    pub zero_offset: f64,

    /// Set the average sampling depth of the absolute encoder (1, 2, 4, 8, 16,
    /// 32, 64, or 128). The default value is 128.
    #[builder(default = 128, setter(transform = |depth: u8| {
        assert!(depth.is_power_of_two() && depth <= 128);
        depth
    }))]
    pub average_depth: u8,

    /// Set the start pulse width of the absolute encoder.
    #[builder(default = -1.0, setter(transform = |width: Duration| width.as_micros() as f64))]
    pub start_pulse_us: f64,

    /// Set the end pulse width of the absolute encoder.
    #[builder(default = -1.0, setter(transform = |width: Duration| width.as_micros() as f64))]
    pub end_pulse_us: f64,

    /// Enable/disable zero-centered reporting of position (-0.5 to 0.5 range
    /// instead of 0 to 1)
    #[builder(default = false)]
    pub zero_centered: bool,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FeedbackSensor {
    None = 0,
    RelativeEncoder = 1,
    AnalogSensor = 2,
    AlternateEncoder = 3,
    AbsoluteEncoder = 4,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, TypedBuilder)]
#[builder(mutators(
    /// Enable position wrapping for the closed loop controller.
    fn position_wrap(&mut self, min: f64, max: f64) {
        self.position_wrap_min = min;
        self.position_wrap_max = max;
        self.position_wrapping = true;
    }

    /// Set the PID gains for the closed loop controller.
    fn pid(&mut self, p: f64, i: f64, d: f64) {
        self.proportional = p;
        self.integral = i;
        self.derivative = d;
    }

    /// Set the PID and feedforward gains for the closed loop controller.
    fn pidf(&mut self, p: f64, i: f64, d: f64, ff: f64) {
        self.proportional = p;
        self.integral = i;
        self.derivative = d;
        self.feedforward = ff;
    }
))]
pub struct ClosedLoopConfig {
    /// The proportional gain of the closed loop controller.
    #[builder(via_mutators, default = 0.0)]
    pub proportional: f64,

    /// The integral gain of the closed loop controller.
    #[builder(via_mutators, default = 0.0)]
    pub integral: f64,

    /// The derivative gain of the closed loop controller.
    #[builder(via_mutators, default = 0.0)]
    pub derivative: f64,

    /// The feedforward gain of the closed loop controller.
    #[builder(via_mutators, default = 0.0)]
    pub feedforward: f64,

    /// The maximum integral accumulator of the closed loop controller.
    #[builder(default = -1.0)]
    pub max_integral: f64,

    /// The integral zone of the closed loop controller.
    #[builder(default = -1.0)]
    pub integral_zone: f64,

    /// The minimum output of the closed loop controller.
    #[builder(default = -1.0, setter(transform = |value: f64| {
        assert!((-1.0..=1.0).contains(&value), "min_output must be between -1.0 and 1.0");
        value
    }))]
    pub min_output: f64,

    /// The maximum output of the closed loop controller.
    #[builder(default = 1.0, setter(transform = |value: f64| {
        assert!((-1.0..=1.0).contains(&value), "max_output must be between -1.0 and 1.0");
        value
    }))]
    pub max_output: f64,

    /// Enable position wrapping for the closed loop controller.
    #[builder(via_mutators, default = false)]
    pub position_wrapping: bool,

    /// The minimum position for position wrapping.
    #[builder(via_mutators, default = -1.0)]
    pub position_wrap_min: f64,

    /// The maximum position for position wrapping.
    #[builder(via_mutators, default = -1.0)]
    pub position_wrap_max: f64,

    /// The feedback sensor to use for the closed loop controller.
    #[builder(default = FeedbackSensor::RelativeEncoder)]
    pub feedback_sensor: FeedbackSensor,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, TypedBuilder)]
pub struct RelativeEncoderConfig {
    /// Set the counts per revolution of the encoder.
    /// This only applies for brushed motors
    #[builder(default = 0)]
    pub counts_per_revolution: u32,

    /// Set the phase of the encoder so that it is in phase with the motor
    /// itself. This only applies for brushed motors
    #[builder(default = false)]
    pub inverted: bool,

    /// Set the conversion factor for the position of the relative encoder.
    /// The native unit is rotations and values will be multiplied by this
    /// conversion factor
    ///
    /// This is NOT RECOMMENDED, you will break uom outputs if you use this
    #[builder(default = 1.0)]
    pub position_factor: f64,

    /// Set the conversion factor for the velocity of the relative encoder.
    /// The native unit is rotations per minute and values will be multiplied by
    /// this conversion factor
    ///
    /// This is NOT RECOMMENDED, you will break uom outputs if you use this
    #[builder(default = 1.0)]
    pub velocity_factor: f64,

    /// Set the sampling depth of the velocity calculation process of the
    /// encoder. This value sets the number of samples in the average for
    /// velocity readings.
    ///
    /// This value must be in the range [1, 64]. The default value is 64.
    #[builder(default = 0, setter(transform = |depth: u8| {
        assert!(depth.is_power_of_two() && depth <= 64);
        depth
    }))]
    pub quadrature_average_depth: u8,

    /// Set the position measurement period used to calculate the velocity of
    /// the encoder. This value must be >= 1ms and <= 100ms, and will be
    /// interpreted in a whole number of ms. The default value is 100ms.
    #[builder(default = 0, setter(transform = |d: Duration| d.as_millis() as u8))]
    pub quadrature_measurement_period: u8,

    /// Set the sampling depth of the velocity calculation process of the
    /// encoder. This value sets the number of samples in the average for
    /// velocity readings. This value must be either 1, 2, 4, or 8
    /// (default).
    #[builder(default = 0, setter(transform = |depth: u8| {
        assert!(depth.is_power_of_two() && depth <= 8);
        depth
    }))]
    pub uvw_average_depth: u8,

    /// Set the position measurement period used to calculate the velocity of
    /// the encoder. This value is in units of milliseconds and must be in a
    /// range [8, 64]. The default value is 32ms. The basic formula to
    /// calculate velocity is change in position / change in time.
    /// This parameter sets the change in time for measurement.
    #[builder(default = 32, setter(transform = |d: Duration| {
        assert!(d.as_millis() >= 8 && d.as_millis() <= 64, "Period must be between 8ms and 64ms");
        d.as_millis() as u8
    }))]
    pub uvw_measurement_period: u8,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, TypedBuilder)]
#[builder(mutators(
    /// Follow the output of another motor controller
    #[mutator(requires = [inverted])]
    fn follow_id(&mut self, leader_id: u8, inverted: bool) {
        self.leader_id = leader_id;
        self.inverted = inverted;
    }

    /// Follow the output of another spark max
    #[mutator(requires = [inverted])]
    fn follow_spark(&mut self, leader: &SparkMax, inverted: bool) {
        self.leader_id = leader.id();
        self.inverted = inverted;
    }

    /// Set the current limit of the motor
    fn current_limit(&mut self, current: ElectricCurrent) {
        self.current_limit = current.get::<amp>();
    }

    /// Set the nominal voltage of the motor
    fn nominal_voltage(&mut self, voltage: ElectricPotential) {
        self.nominal_voltage = voltage.get::<volt>();
    }
))]
pub struct MotorConfig {
    /// The type of motor
    pub motor_type: MotorType,

    /// The idle mode of the motor
    #[builder(default = IdleMode::Brake)]
    pub idle_mode: IdleMode,

    /// The id of the leader motor when in follow mode
    #[builder(via_mutators, default = 0)]
    pub leader_id: u8,

    /// Whether the motor is inverted
    #[builder(default = false)]
    pub inverted: bool,

    /// The current limit of the motor
    #[builder(via_mutators, default = 0.0)]
    pub current_limit: f64,

    /// The nominal voltage if using voltage compensation
    #[builder(via_mutators, default = 0.0)]
    pub nominal_voltage: f64,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SparkMaxConfig {
    pub absolute_encoder: AbsoluteEncoderConfig,
    pub closed_loop: ClosedLoopConfig,
    pub relative_encoder: RelativeEncoderConfig,
    pub motor: MotorConfig,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotorType {
    Brushed = 0,
    Brushless = 1,
}

#[ffi_type(namespace = "ffi::device::spark::config")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdleMode {
    /// The motor will hold its position when not powered
    Brake = 0,
    /// The motor will coast when not powered
    Coast = 1,
}

impl Default for AbsoluteEncoderConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Default for ClosedLoopConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Default for RelativeEncoderConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[cfg(feature = "build")]
pub(super) fn __ffi_inventory(builder: InventoryBuilder) -> InventoryBuilder {
    builder
        .register(extra_type!(FeedbackSensor))
        .register(extra_type!(MotorType))
        .register(extra_type!(AbsoluteEncoderConfig))
        .register(extra_type!(ClosedLoopConfig))
        .register(extra_type!(RelativeEncoderConfig))
        .register(extra_type!(SparkMaxConfig))
}
