#pragma once

#include <ffi/spark.h>
#include <rev/SparkMax.h>
#include <map>
#include <memory>

using namespace rev::spark;
namespace spark_ffi = ffi::spark;

class SparkMaxContainer
{
public:
	void HandleCommand(uint8_t can_id, const spark_ffi::Command *command);

private:
	void HandleCreate(uint8_t can_id, SparkBase::MotorType motor_type);
	static SparkBase::MotorType Convert(spark_ffi::config::MotorType motor_type);

	std::map<uint8_t, std::unique_ptr<SparkMax>> m_motors = {};
};