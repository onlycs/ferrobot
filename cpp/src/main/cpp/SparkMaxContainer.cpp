#include "SparkMaxContainer.h"
#include <iostream>

void SparkMaxContainer::HandleCommand(uint8_t can_id, const ffi::SparkMaxCommand *command)
{
	switch (command->kind)
	{
	case ffi::CommandType::Create:
		const ffi::MotorType *motor_type = (const ffi::MotorType *)command->data;
		HandleCreate(can_id, Convert(*motor_type));
		break;
	default:
		std::cerr << "[ERROR] Unknown command type: " << (int)command->kind << std::endl;
		break;
	}
}

void SparkMaxContainer::HandleCreate(uint8_t can_id, spark::SparkMax::MotorType motor_type)
{
	if (m_motors.find(can_id) == m_motors.end())
	{
		m_motors[can_id] = std::make_unique<spark::SparkMax>(can_id, motor_type);
	}
	else
	{
		std::cerr << "[ERROR] Motor with ID " << (int)can_id << " already exists." << std::endl;
		std::cerr << "This should have been checked on the Rust side. Please submit an issue." << std::endl;
	}
}

spark::SparkBase::MotorType SparkMaxContainer::Convert(ffi::MotorType motor_type)
{
	switch (motor_type)
	{
	case ffi::MotorType::Brushed:
		return spark::SparkBase::MotorType::kBrushed;
	case ffi::MotorType::Brushless:
		return spark::SparkBase::MotorType::kBrushless;
	default:
		std::cerr << "[ERROR] Unknown motor type: " << (int)motor_type << std::endl;
		std::cerr << "Defaulting to brushless" << std::endl;
		return spark::SparkBase::MotorType::kBrushless;
	}
}