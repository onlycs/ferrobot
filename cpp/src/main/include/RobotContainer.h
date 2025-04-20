#pragma once

#include <SparkMaxContainer.h>
#include <ffi/device.h>
#include <ffi/ferrobot.h>

namespace device = ffi::device;

class RobotContainer
{
public:
	RobotContainer();
	ffi::Response HandleCommand(device::Command command);

private:
	SparkMaxContainer m_sparkMaxContainer = SparkMaxContainer();
};