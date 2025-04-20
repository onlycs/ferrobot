#include "RobotContainer.h"
#include <iostream>

RobotContainer::RobotContainer() {}

ffi::Response RobotContainer::HandleCommand(device::Command *command)
{
	bool ok = true;
	void *response_ptr = nullptr;

	switch (command->device.kind)
	{
	case device::Type::SparkMax:
	{
		std::optional<spark_ffi::Error> error = m_sparkMaxContainer.HandleCommand(command->device.id, (const spark_ffi::Command *)command->command);
		if (error.has_value())
		{
			ok = false;
			response_ptr = malloc(sizeof(spark_ffi::Error));
			*(spark_ffi::Error *)response_ptr = error.value();
		}
		break;
	}
	default:
	{
		ok = false;
	}
	}

	return ffi::Response{
		.ok = ok,
		.data = response_ptr,
	};
}