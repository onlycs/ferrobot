// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

#include "Robot.h"
#include "studica/AHRS.h"
#include "rev/config/AbsoluteEncoderConfig.h"
#include "iostream"

Robot::Robot()
{
	ffi::start_thread();
}

/**
 * This function is called every 20 ms, no matter the mode. Use
 * this for items like diagnostics that you want to run during disabled,
 * autonomous, teleoperated and test.
 *
 * <p> This runs after the mode specific periodic functions, but before
 * LiveWindow and SmartDashboard integrated updating.
 */
void Robot::RobotPeriodic()
{
	ffi::DeviceCommands commands = ffi::collect();
	for (size_t i = 0; i < commands.len; i++)
	{
		const ffi::DeviceCommand &command = commands.data[i];
		switch (command.device.kind)
		{
		case ffi::DeviceType::SparkMax:
			m_sparkMaxContainer.HandleCommand(command.device.id, (const ffi::SparkMaxCommand *)command.command);
			break;
		default:
			std::cerr << "Unknown device type" << std::endl;
			break;
		}
	}

	ffi::device_commands_free(commands);
}

/**
 * This function is called once each time the robot enters Disabled mode. You
 * can use it to reset any subsystem information you want to clear when the
 * robot is disabled.
 */
void Robot::DisabledInit() {}

void Robot::DisabledPeriodic() {}

/**
 * This autonomous runs the autonomous command selected by your {@link
 * RobotContainer} class.
 */
void Robot::AutonomousInit() {}

void Robot::AutonomousPeriodic() {}

void Robot::TeleopInit() {}

/**
 * This function is called periodically during operator control.
 */
void Robot::TeleopPeriodic() {}

/**
 * This function is called periodically during test mode.
 */
void Robot::TestPeriodic() {}

/**
 * This function is called once when the robot is first started up.
 */
void Robot::SimulationInit() {}

/**
 * This function is called periodically whilst in simulation.
 */
void Robot::SimulationPeriodic() {}

#ifndef RUNNING_FRC_TESTS
int main()
{
	return frc::StartRobot<Robot>();
}
#endif
