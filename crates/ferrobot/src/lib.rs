#![allow(clippy::arc_with_non_send_sync)]

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use async_std::task;
use context::Context;
use ffi::DeviceCommands;

extern crate async_std;
extern crate cxx;
extern crate typed_builder;

pub mod context;
pub mod device;
mod ffi;
mod prelude;

// late-init globals
static mut QUEUE: Option<Arc<Mutex<VecDeque<device::ffi::DeviceCommand>>>> = None;

async fn main() {}

// starts the main thread
#[unsafe(no_mangle)]
extern "C" fn start_thread() {
    // initialize default queue
    unsafe { QUEUE = Some(Arc::new(Mutex::new(VecDeque::new()))) }

    // spawn the main thread
    thread::spawn(|| task::block_on(main()));
}

#[unsafe(no_mangle)]
#[allow(static_mut_refs, clippy::await_holding_lock)]
extern "C" fn supply(context: context::ContextFFI) {
    task::spawn(Context::instance().replace(context.devices));
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
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
