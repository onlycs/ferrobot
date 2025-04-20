#![feature(error_generic_member_access)]
#![allow(clippy::arc_with_non_send_sync)]

extern crate async_std;
extern crate interoptopus;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate log;
extern crate thiserror;
extern crate typed_builder;
extern crate uom;

mod context;
pub mod device;
mod ffi;
pub mod prelude;

use std::thread;

use async_std::task;
use context::Context;
use prelude::*;

async fn main() {
    println!("Hello World!");
}

#[allow(unused)]
#[ffi_function(namespace = "ffi")]
extern "C" fn start_thread() {
    // spawn the main thread
    thread::spawn(|| task::block_on(main()));
}

#[allow(static_mut_refs, clippy::await_holding_lock, unused)]
#[ffi_function(namespace = "ffi")]
fn supply(context: context::ContextFFI) {
    task::spawn(Context::instance().replace(context.devices));
}

#[cfg(feature = "build")]
pub mod build {
    use interoptopus::{
        backend::NamespaceMappings,
        function,
        inventory::{Inventory, InventoryBuilder},
    };
    use interoptopus_backend_c::{EnumVariants, Interop, InteropBuilder, NameCase};

    fn __ffi_inventory() -> Inventory {
        let mut builder = InventoryBuilder::new();

        builder = crate::device::__ffi_inventory(builder);
        builder = crate::ffi::__ffi_inventory(builder);
        builder = crate::context::__ffi_inventory(builder);

        builder
            .register(function!(crate::supply))
            .register(function!(crate::start_thread))
            .validate()
            .build()
    }

    pub fn __ffi_interop() -> Interop {
        InteropBuilder::new()
            .inventory(__ffi_inventory())
            .const_naming(NameCase::ShoutySnake)
            .enum_variant_naming(NameCase::UpperCamel)
            .enum_variant_style(EnumVariants::VariantName)
            .function_parameter_naming(NameCase::Snake)
            .type_naming(NameCase::UpperCamel)
            .cpp(true)
            .namespace_paths(NamespaceMappings::new("ffi/ferrobot.h"))
            .build()
            .expect("Failed to build interop")
    }
}
