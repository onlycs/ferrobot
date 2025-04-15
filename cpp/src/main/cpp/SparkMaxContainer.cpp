#include "SparkMaxContainer.h"
#include <iostream>

void SparkMaxContainer::HandleCommand(uint8_t can_id, const spark_ffi::Command *command)
{
	switch (command->kind)
	{
	case spark_ffi::CommandType::CommandTypeCreate: {
		HandleCreate(can_id, Convert(*(const spark_ffi::config::MotorType *)command->data));
		break;
	}
	default: {
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

SparkMax::MotorType SparkMaxContainer::Convert(spark_ffi::config::MotorType motor_type)
{
	switch (motor_type)
	{
	case spark_ffi::config::MotorType::MotorTypeBrushed:
		return SparkMax::MotorType::kBrushed;
	case spark_ffi::config::MotorType::MotorTypeBrushless:
		return SparkMax::MotorType::kBrushless;
	default:
		std::cerr << "[ERROR] Unknown motor type: " << (int)motor_type << std::endl;
		std::cerr << "Defaulting to brushless" << std::endl;
		return SparkMax::MotorType::kBrushless;
	}
}
