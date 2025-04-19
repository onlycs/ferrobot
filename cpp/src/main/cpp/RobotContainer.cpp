#include "RobotContainer.h"
#include <iostream>

RobotContainer::RobotContainer() {}

void *RobotContainer::HandleCommand(device::Command command)
{
	void *response_ptr = nullptr;

	switch (command.device.kind)
	{
	case device::Type::SparkMax:
	{
		response_ptr = malloc(sizeof(spark_ffi::Response));
		*(spark_ffi::Response *)response_ptr = m_sparkMaxContainer.HandleCommand(command.device.id, (const spark_ffi::Command *)command.command);
		break;
	}
	default:
	{
		std::cerr << "[ERROR] Unknown device type: " << (int)command.device.kind << std::endl;
		return nullptr;
	}
	}

	return (void *)response_ptr;
}