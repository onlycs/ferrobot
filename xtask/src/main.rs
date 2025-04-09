#![feature(error_generic_member_access)]

mod args;
mod error;
mod paths;
mod util;

use std::{env, fs, process::Command};

use args::{Arguments, Operation};
use clap::Parser;
use error::TaskResult;
use paths::LIBSTATIC;

const ATHENA_TARGET: &str = "arm-unknown-linux-gnueabi";
const HOST_TARGET: &str = "x86_64-unknown-linux-gnu";

fn run(args: Arguments) -> TaskResult {
    match args.operation {
        Operation::Build { mode } => {
            // clear libstatic
            if LIBSTATIC.exists() {
                fs::remove_dir_all(&*LIBSTATIC)?;
            }
            fs::create_dir_all(&*LIBSTATIC)?;

            // run cxxbridge
            env::set_current_dir(&*paths::WORKSPACE)?;

            util::exec(
                Command::new("cxxbridge")
                    .arg("src/ffi.rs")
                    .arg("--header")
                    .arg("-o")
                    .arg("cpp/src/main/include/ffi.h"),
            )?;

            util::exec(
                Command::new("cxxbridge")
                    .arg("src/ffi.rs")
                    .arg("-o")
                    .arg("cpp/src/main/cpp/ffi.cpp"),
            )?;

            // run cargo build
            util::cargo(&["build", "--target", ATHENA_TARGET], mode)?;
            util::cargo(&["build", "--target", HOST_TARGET], mode)?;

            let athena_dir = paths::TARGET.join(ATHENA_TARGET).join(mode.to_string());
            let librobot_athena = athena_dir.join("librobot.a");
            let librobot_dest_athena = paths::LIBSTATIC.join("librobot_athena.a");
            fs::copy(&librobot_athena, &librobot_dest_athena)?;

            let x64_dir = paths::TARGET.join(HOST_TARGET).join(mode.to_string());
            let librobot_x64 = x64_dir.join("librobot.a");
            let librobot_dest_x64 = paths::LIBSTATIC.join("librobot_x64.a");
            fs::copy(&librobot_x64, &librobot_dest_x64)?;

            // cd into ./cpp
            env::set_current_dir(&*paths::CPP)?;

            // run gradlew build with gradlew in current dir
            util::gradle(&["build"])?;
        }
        Operation::Simulate { mode } => {
            run(Arguments {
                operation: Operation::Build { mode },
            })?;

            util::gradle(&["simulateNativeRelease"])?;
        }
        Operation::Deploy { mode } => {
            run(Arguments {
                operation: Operation::Build { mode },
            })?;

            util::gradle(&["deploy"])?;
        }
    }

    Ok(())
}

fn main() -> TaskResult {
    run(Arguments::parse())
}
