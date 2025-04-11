#pragma once

#include <ffi.h>
#include <rev/SparkMax.h>
#include <map>
#include <memory>

using namespace rev;

class SparkMaxContainer
{
public:
	void HandleCommand(uint8_t can_id, const ffi::SparkMaxCommand *command);

private:
	void HandleCreate(uint8_t can_id, spark::SparkMax::MotorType motor_type);
	static spark::SparkBase::MotorType Convert(ffi::MotorType motor_type);

	std::map<uint8_t, std::unique_ptr<spark::SparkMax>> m_motors = {};
};