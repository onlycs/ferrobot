use async_std::task;
use std::sync::{
    Arc, Mutex,
    atomic::{self, AtomicPtr},
};

extern crate async_std;
extern crate cxx;

mod ffi;

// late-init globals
static mut JOIN_HANDLE: Option<Arc<Mutex<task::JoinHandle<()>>>> = None;
static mut CONTEXT: Option<Arc<AtomicPtr<ffi::Context>>> = None;

async fn main() {}

// starts the main thread
fn start_thread() {
    // initialize default context
    unsafe {
        let ptr = Box::into_raw(Box::new(ffi::Context::default()));
        CONTEXT = Some(Arc::new(AtomicPtr::new(ptr)));
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
