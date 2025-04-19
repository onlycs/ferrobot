// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

#pragma once

#include <optional>

#include <frc/TimedRobot.h>
#include <RobotContainer.h>

using namespace rev;

class Robot : public frc::TimedRobot
{
public:
	inline static RobotContainer m_robotContainer = RobotContainer();

	Robot();
	void RobotPeriodic() override;
	void DisabledInit() override;
	void DisabledPeriodic() override;
	void AutonomousInit() override;
	void AutonomousPeriodic() override;
	void TeleopInit() override;
	void TeleopPeriodic() override;
	void TestPeriodic() override;
	void SimulationInit() override;
	void SimulationPeriodic() override;

private:
};

#ifdef __cplusplus
extern "C"
{
#endif
	// C function to start the robot
	void *handle_command(device::Command command);

#ifdef __cplusplus
}
#endif