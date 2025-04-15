#![feature(error_generic_member_access, let_chains)]

extern crate cbindgen;
extern crate clap;
extern crate lazy_static;
extern crate thiserror;

mod args;
mod error;
mod paths;
mod util;

use std::{env, fs};

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

            // run interoptopus
            let mut interop = ferrobot::__ffi_interop();

            // write to file
            interop.write_all(&paths::INCLUDE)?;

            // run cargo build
            util::cargo(&["build", "--target", ATHENA_TARGET], mode)?;
            util::cargo(&["build", "--target", HOST_TARGET], mode)?;

            // copy static libraries
            let athena_dir = paths::TARGET.join(ATHENA_TARGET).join(mode.to_string());
            let libferrobot_athena = athena_dir.join("libferrobot.a");
            let libferrobot_dest_athena = paths::LIBSTATIC.join("libferrobot_athena.a");
            fs::copy(&libferrobot_athena, &libferrobot_dest_athena)?;

            let x64_dir = paths::TARGET.join(HOST_TARGET).join(mode.to_string());
            let libferrobot_x64 = x64_dir.join("libferrobot.a");
            let libferrobot_dest_x64 = paths::LIBSTATIC.join("libferrobot_x64.a");
            fs::copy(&libferrobot_x64, &libferrobot_dest_x64)?;

            // run gradlew build with gradlew in cpp dir
            env::set_current_dir(&*paths::CPP)?;
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
