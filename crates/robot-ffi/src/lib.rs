#![allow(clippy::arc_with_non_send_sync)]

use async_std::task;
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{self, AtomicPtr},
    },
};

extern crate async_std;
extern crate cxx;

mod ffi;

// late-init globals
static mut JOIN_HANDLE: Option<Arc<Mutex<task::JoinHandle<()>>>> = None;
static mut CONTEXT: Option<Arc<AtomicPtr<ffi::Context>>> = None;
static mut QUEUE: Option<Arc<Mutex<VecDeque<ffi::DeviceCommand>>>> = None;

async fn main() {}

// starts the main thread
fn start_thread() {
    // initialize default context and queue
    unsafe {
        let ptr = Box::into_raw(Box::new(ffi::Context::default()));
        CONTEXT = Some(Arc::new(AtomicPtr::new(ptr)));
        QUEUE = Some(Arc::new(Mutex::new(VecDeque::new())));
    }

    // spawn the main thread
    let handle = async_std::task::spawn(main());

    // store thread handle for later use (i.e. to kill)
    unsafe { JOIN_HANDLE = Some(Arc::new(Mutex::new(handle))) }
}

fn supply(context: ffi::Context) {
    unsafe {
        // context is initialized in start_thread(), which should always be run first
        let Some(ref ctx) = CONTEXT else { return };

        let ptr = Box::into_raw(Box::new(context)); // pin to heap "forever"
        let old = ctx.swap(ptr, atomic::Ordering::Relaxed); // swap the old context
        drop(Box::from_raw(old)); // free the old context (memory leak bad)
    }
}

#[allow(static_mut_refs)]
fn collect() -> Vec<ffi::DeviceCommand> {
    let queue = match unsafe { QUEUE.as_ref() } {
        Some(queue) => queue,
        None => return vec![], // shouldn't happen, QUEUE is initialized in start_thread()
    };

    let mut lock = match queue.lock() {
        Ok(queue) => queue,
        Err(poisoned) => poisoned.into_inner(),
    };

    lock.drain(..).collect()
}
