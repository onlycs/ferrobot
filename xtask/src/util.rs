use crate::error::TaskResult;
use crate::paths;
use crate::{args::BuildMode, error::TaskError};
use std::{env, process};

pub fn exec(mut command: process::Command) -> TaskResult {
    let status = command.status()?;

    if !status.success() {
        return Err(TaskError::Command {
            command: command
                .get_args()
                .map(|s| s.to_str().unwrap().to_string())
                .collect(),
        });
    }

    Ok(())
}

pub fn cargo(args: &[&str], with_mode: BuildMode) -> TaskResult {
    env::set_current_dir(&*paths::CPP).unwrap();

    let mut cmd = process::Command::new("cargo");
    cmd.args(args);

    if with_mode == BuildMode::Release {
        cmd.arg("--release");
    }

    exec(cmd)
}

pub fn gradle(args: &[&str]) -> TaskResult {
    env::set_current_dir(&*paths::CPP)?;

    let mut cmd = process::Command::new("./gradlew");
    cmd.args(args);

    exec(cmd)
}
