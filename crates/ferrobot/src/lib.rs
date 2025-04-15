#![allow(clippy::arc_with_non_send_sync)]

extern crate async_std;
extern crate cxx;
extern crate typed_builder;

pub mod context;
pub mod device;
mod ffi;

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use async_std::task;
use context::Context;
use ffi::DeviceCommands;
use interoptopus::{
    backend::NamespaceMappings,
    ffi_function, function,
    inventory::{Inventory, InventoryBuilder},
};
use interoptopus_backend_c::{Interop, InteropBuilder, NameCase};

// late-init globals
static mut QUEUE: Option<Arc<Mutex<VecDeque<device::ffi::DeviceCommand>>>> = None;

async fn main() {
    println!("Hello World!");
}

// starts the main thread
#[allow(unused)]
#[ffi_function(namespace = "ffi")]
extern "C" fn start_thread() {
    // initialize default queue
    unsafe { QUEUE = Some(Arc::new(Mutex::new(VecDeque::new()))) }

    // spawn the main thread
    thread::spawn(|| task::block_on(main()));
}

#[allow(static_mut_refs, clippy::await_holding_lock, unused)]
#[ffi_function(namespace = "ffi")]
extern "C" fn supply(context: context::ContextFFI) {
    task::spawn(Context::instance().replace(context.devices));
}

#[allow(static_mut_refs, unused)]
#[ffi_function(namespace = "ffi")]
extern "C" fn collect() -> ffi::DeviceCommands {
    let queue = match unsafe { QUEUE.as_ref() } {
        Some(queue) => queue,
        None => return DeviceCommands::new(vec![]),
    };

    let mut lock = match queue.lock() {
        Ok(queue) => queue,
        Err(poisoned) => poisoned.into_inner(),
    };

    DeviceCommands::new(lock.drain(..).collect())
}

fn __ffi_inventory() -> Inventory {
    let mut builder = InventoryBuilder::new();

    builder = device::__ffi_inventory(builder);
    builder = ffi::__ffi_inventory(builder);
    builder = context::__ffi_inventory(builder);

    builder
        .register(function!(collect))
        .register(function!(supply))
        .register(function!(start_thread))
        .validate()
        .build()
}

#[cfg(feature = "build")]
pub fn __ffi_interop() -> Interop {
    InteropBuilder::new()
        .inventory(__ffi_inventory())
        .const_naming(NameCase::ShoutySnake)
        .enum_variant_naming(NameCase::UpperCamel)
        .function_parameter_naming(NameCase::Snake)
        .type_naming(NameCase::UpperCamel)
        .namespace_paths(NamespaceMappings::new("ffi/ferrobot.h"))
        .build()
        .expect("Failed to build interop")
}
