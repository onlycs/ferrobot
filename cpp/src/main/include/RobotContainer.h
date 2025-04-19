#pragma once

#include <SparkMaxContainer.h>
#include <ffi/device.h>

namespace device = ffi::device;

class RobotContainer
{
public:
	RobotContainer();
	void *HandleCommand(device::Command command);

private:
	SparkMaxContainer m_sparkMaxContainer = SparkMaxContainer();
};