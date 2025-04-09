#![allow(clippy::arc_with_non_send_sync)]

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use async_std::{channel, task};
use context::Context;
use ffi::DeviceCommands;

extern crate async_std;
extern crate cxx;
extern crate typed_builder;

pub mod context;
pub mod device;
mod ffi;

// late-init globals
static mut QUEUE: Option<Arc<Mutex<VecDeque<device::ffi::DeviceCommand>>>> = None;
static mut SENDER: Option<Arc<Mutex<channel::Sender<Context>>>> = None;

async fn main() {
    let (sender, receiver) = channel::unbounded();

    unsafe { SENDER = Some(Arc::new(Mutex::new(sender))) }
}

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
    let context = Context::new(context.devices, unsafe {
        Arc::clone(QUEUE.as_ref().unwrap())
    });

    async fn send(ctx: Context) {
        let sender = unsafe { SENDER.as_ref().unwrap().clone() };
        let sender = &mut *match sender.lock() {
            Ok(sender) => sender,
            Err(poisoned) => poisoned.into_inner(),
        };

        sender.send(ctx).await.unwrap();
    }

    // send context to the main thread
    thread::spawn(move || {
        task::block_on(send(context));
    });
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
