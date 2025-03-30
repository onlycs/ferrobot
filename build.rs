#[cfg(not(target_os = "windows"))]
fn main() {
    use std::{env, process::Command};

    Command::new("cxxbridge")
        .arg("src/bridge.rs")
        .arg("--header")
        .arg("-o")
        .arg("cpp/src/main/include/ffi.h")
        .status()
        .expect("Failed to generate C++ header file");

    Command::new("cxxbridge")
        .arg("src/bridge.rs")
        .arg("-o")
        .arg("cpp/src/main/cpp/ffi.cpp")
        .status()
        .expect("Failed to generate C++ static library");

    // cxx_build::bridge("src/bridge.rs")
    //     .target("arm-unknown-linux-gnueabi")
    //     .out_dir("cpp/src/main/libstatic")
    //     .compile("robot");

    println!("cargo:rerun-if-changed=src/**/*");
    println!("cargo:rerun-if-changed=cpp/src/**/*");
}
