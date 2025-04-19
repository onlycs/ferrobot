#include "SparkMaxContainer.h"
#include <iostream>

spark_ffi::Response SparkMaxContainer::HandleCommand(uint8_t can_id, const spark_ffi::Command *command)
{
	try
	{
		switch (command->kind)
		{
		case spark_ffi::CommandType::Create:
		{
			HandleCreate(can_id, (const spark_ffi::config::SparkMaxConfig *)command->data);
			break;
		}
		default:
		{
			std::cerr << "[ERROR] Unknown command type: " << (int)command->kind << std::endl;
			break;
		}
		}
	}
	catch (const spark_ffi::Response &err)
	{
		return err;
	}

	return spark_ffi::Response::Ok;
}

void SparkMaxContainer::HandleCreate(uint8_t can_id, const spark_ffi::config::SparkMaxConfig *config)
{
	if (m_motors.contains(can_id))
	{
		throw spark_ffi::Response::MotorExists;
	}

	std::unique_ptr<SparkMaxConfig> converted_config = Convert(config);
	SparkBase::MotorType motor_type = Convert(&config->motor.motor_type);
	std::unique_ptr<SparkMax> motor = std::make_unique<SparkMax>(can_id, motor_type);
	motor->Configure(*converted_config, SparkBase::ResetMode::kResetSafeParameters, SparkBase::PersistMode::kNoPersistParameters);
	m_motors.emplace(can_id, std::move(motor));
}

SparkMax::MotorType SparkMaxContainer::Convert(const spark_ffi::config::MotorType *motor_type)
{
	switch (*motor_type)
	{
	case spark_ffi::config::MotorType::Brushed:
		return SparkMax::MotorType::kBrushed;
	case spark_ffi::config::MotorType::Brushless:
		return SparkMax::MotorType::kBrushless;
	default:
		throw spark_ffi::Response::BadConfig;
	}
}

ClosedLoopConfig::FeedbackSensor SparkMaxContainer::Convert(spark_ffi::config::FeedbackSensor sensor)
{
	switch (sensor)
	{
	case spark_ffi::config::FeedbackSensor::None:
		return ClosedLoopConfig::FeedbackSensor::kNoSensor;
	case spark_ffi::config::FeedbackSensor::RelativeEncoder:
		return ClosedLoopConfig::FeedbackSensor::kPrimaryEncoder;
	case spark_ffi::config::FeedbackSensor::AnalogSensor:
		return ClosedLoopConfig::FeedbackSensor::kAnalogSensor;
	case spark_ffi::config::FeedbackSensor::AlternateEncoder:
		return ClosedLoopConfig::FeedbackSensor::kAlternateOrExternalEncoder;
	case spark_ffi::config::FeedbackSensor::AbsoluteEncoder:
		return ClosedLoopConfig::FeedbackSensor::kAbsoluteEncoder;
	default:
		throw spark_ffi::Response::BadConfig;
	}
}

SparkBaseConfig::IdleMode SparkMaxContainer::Convert(spark_ffi::config::IdleMode mode)
{
	switch (mode)
	{
	case spark_ffi::config::IdleMode::Coast:
		return SparkBaseConfig::IdleMode::kCoast;
	case spark_ffi::config::IdleMode::Brake:
		return SparkBaseConfig::IdleMode::kBrake;
	default:
		throw spark_ffi::Response::BadConfig;
	}
}

std::unique_ptr<SparkMaxConfig> SparkMaxContainer::Convert(const spark_ffi::config::SparkMaxConfig *config)
{
	std::unique_ptr<SparkMaxConfig> converted_uniq = std::make_unique<SparkMaxConfig>();
	SparkMaxConfig *converted = converted_uniq.get();

	// absolute encoder
	converted->absoluteEncoder
		.Inverted(config->absolute_encoder.inverted)
		.PositionConversionFactor(config->absolute_encoder.position_factor)
		.VelocityConversionFactor(config->absolute_encoder.velocity_factor)
		.ZeroOffset(config->absolute_encoder.zero_offset)
		.AverageDepth(config->absolute_encoder.average_depth)
		.ZeroCentered(config->absolute_encoder.zero_centered);

	if (config->absolute_encoder.start_pulse_us != -1.0 && config->absolute_encoder.end_pulse_us != -1.0)
	{
		converted->absoluteEncoder
			.StartPulseUs(config->absolute_encoder.start_pulse_us)
			.EndPulseUs(config->absolute_encoder.end_pulse_us);
	}

	// closed loop
	converted->closedLoop
		.Pidf(
			config->closed_loop.proportional,
			config->closed_loop.integral,
			config->closed_loop.derivative,
			config->closed_loop.feedforward)
		.MinOutput(config->closed_loop.min_output)
		.MaxOutput(config->closed_loop.max_output)
		.SetFeedbackSensor(Convert(config->closed_loop.feedback_sensor));

	if (config->closed_loop.max_integral != -1.0)
	{
		converted->closedLoop.IMaxAccum(config->closed_loop.max_integral);
	}

	if (config->closed_loop.integral_zone != -1.0)
	{
		converted->closedLoop.IZone(config->closed_loop.integral_zone);
	}

	if (config->closed_loop.position_wrapping)
	{
		converted->closedLoop
			.PositionWrappingEnabled(config->closed_loop.position_wrapping)
			.PositionWrappingMinInput(config->closed_loop.position_wrap_min)
			.PositionWrappingMaxInput(config->closed_loop.position_wrap_max);
	}

	// relative encoder
	converted->encoder
		.Inverted(config->relative_encoder.inverted)
		.PositionConversionFactor(config->relative_encoder.position_factor)
		.VelocityConversionFactor(config->relative_encoder.velocity_factor);

	if (config->relative_encoder.counts_per_revolution != 0)
	{
		converted->encoder.CountsPerRevolution(config->relative_encoder.counts_per_revolution);
	}

	if (config->relative_encoder.quadrature_average_depth != 0)
	{
		converted->encoder.QuadratureAverageDepth(config->relative_encoder.quadrature_average_depth);
	}

	if (config->relative_encoder.quadrature_measurement_period != 0)
	{
		converted->encoder.QuadratureMeasurementPeriod(config->relative_encoder.quadrature_measurement_period);
	}

	if (config->relative_encoder.uvw_average_depth != 0)
	{
		converted->encoder.UvwAverageDepth(config->relative_encoder.uvw_average_depth);
	}

	if (config->relative_encoder.uvw_measurement_period != 0)
	{
		converted->encoder.UvwMeasurementPeriod(config->relative_encoder.uvw_measurement_period);
	}

	// motor
	converted->SetIdleMode(Convert(config->motor.idle_mode));

	// inverted means different things based on follow mode or not
	if (config->motor.leader_id != 0)
	{
		converted->Follow(config->motor.leader_id, config->motor.inverted);
	}
	else
	{
		converted->Inverted(config->motor.inverted);
	}

	if (config->motor.current_limit != 0.0)
	{
		converted->SmartCurrentLimit(config->motor.current_limit);
	}

	if (config->motor.nominal_voltage != 0.0)
	{
		converted->VoltageCompensation(config->motor.nominal_voltage);
	}

	return converted_uniq;
}