#pragma once

#include <ffi/device/spark.h>
#include <ffi/device/spark/config.h>
#include <rev/SparkMax.h>
#include <rev/config/SparkMaxConfig.h>
#include <map>
#include <memory>

using namespace rev::spark;
namespace spark_ffi = ffi::device::spark;

class SparkMaxContainer
{
public:
	void HandleCommand(uint8_t can_id, const spark_ffi::Command *command);

private:
	void HandleCreate(uint8_t can_id, const spark_ffi::config::SparkMaxConfig *config);

	static SparkBase::MotorType Convert(const spark_ffi::config::MotorType *motor_type);
	static std::unique_ptr<SparkMaxConfig> Convert(const spark_ffi::config::SparkMaxConfig *config);
	static ClosedLoopConfig::FeedbackSensor Convert(spark_ffi::config::FeedbackSensor sensor);
	static SparkBaseConfig::IdleMode Convert(spark_ffi::config::IdleMode mode);

	std::map<uint8_t, std::unique_ptr<SparkMax>> m_motors = {};
};