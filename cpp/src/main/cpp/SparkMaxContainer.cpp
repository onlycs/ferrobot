#include "SparkMaxContainer.h"
#include <iostream>

void SparkMaxContainer::HandleCommand(uint8_t can_id, const spark_ffi::Command *command)
{
	switch (command->kind)
	{
	case spark_ffi::CommandType::CommandTypeCreate:
	{
		HandleCreate(can_id, Convert((const spark_ffi::config::MotorType *)command->data));
		break;
	}
	case spark_ffi::CommandType::CommandTypeConfigure:
	{
		HandleConfigure(can_id, Convert((const spark_ffi::config::Config *)command->data));
		break;
	}
	default:
	{
		std::cerr << "[ERROR] Unknown command type: " << (int)command->kind << std::endl;
		break;
	}
	}
}

void SparkMaxContainer::HandleCreate(uint8_t can_id, SparkMax::MotorType motor_type)
{
	if (m_motors.find(can_id) == m_motors.end())
	{
		m_motors[can_id] = std::make_unique<SparkMax>(can_id, motor_type);
	}
	else
	{
		std::cerr << "[ERROR] Motor with ID " << (int)can_id << " already exists." << std::endl;
		std::cerr << "This should have been checked on the Rust side. Please submit an issue." << std::endl;
	}
}

void SparkMaxContainer::HandleConfigure(uint8_t can_id, std::unique_ptr<SparkMaxConfig> config)
{
	auto it = m_motors.find(can_id);
	if (it != m_motors.end())
	{
		SparkMax *motor = it->second.get();
		motor->Configure(*config, SparkMax::ResetMode::kResetSafeParameters, SparkMax::PersistMode::kPersistParameters);
	}
	else
	{
		std::cerr << "[ERROR] Motor with ID " << (int)can_id << " does not exist." << std::endl;
	}
}

SparkMax::MotorType SparkMaxContainer::Convert(const spark_ffi::config::MotorType *motor_type)
{
	switch (*motor_type)
	{
	case spark_ffi::config::MotorType::MotorTypeBrushed:
		return SparkMax::MotorType::kBrushed;
	case spark_ffi::config::MotorType::MotorTypeBrushless:
		return SparkMax::MotorType::kBrushless;
	default:
		std::cerr << "[ERROR] Unknown motor type: " << (int)*motor_type << std::endl;
		std::cerr << "Defaulting to brushless" << std::endl;
		return SparkMax::MotorType::kBrushless;
	}
}

ClosedLoopConfig::FeedbackSensor SparkMaxContainer::Convert(spark_ffi::config::FeedbackSensor sensor)
{
	switch (sensor)
	{
	case spark_ffi::config::FeedbackSensor::FeedbackSensorNone:
		return ClosedLoopConfig::FeedbackSensor::kNoSensor;
	case spark_ffi::config::FeedbackSensor::FeedbackSensorRelativeEncoder:
		return ClosedLoopConfig::FeedbackSensor::kPrimaryEncoder;
	case spark_ffi::config::FeedbackSensor::FeedbackSensorAnalogSensor:
		return ClosedLoopConfig::FeedbackSensor::kAnalogSensor;
	case spark_ffi::config::FeedbackSensor::FeedbackSensorAlternateEncoder:
		return ClosedLoopConfig::FeedbackSensor::kAlternateOrExternalEncoder;
	case spark_ffi::config::FeedbackSensor::FeedbackSensorAbsoluteEncoder:
		return ClosedLoopConfig::FeedbackSensor::kAbsoluteEncoder;
	default:
		std::cerr << "[ERROR] Unknown feedback sensor: " << (int)sensor << std::endl;
		std::cerr << "Defaulting to relative encoder" << std::endl;
		return ClosedLoopConfig::FeedbackSensor::kPrimaryEncoder;
	}
}

std::unique_ptr<SparkMaxConfig> SparkMaxContainer::Convert(const spark_ffi::config::Config *config)
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

	return converted_uniq;
}