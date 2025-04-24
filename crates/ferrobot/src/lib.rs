#![feature(error_generic_member_access, downcast_unchecked)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::arc_with_non_send_sync,
    incomplete_features,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    async_fn_in_trait,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]

extern crate async_std;
extern crate interoptopus;
extern crate libc;
extern crate log;
extern crate thiserror;
extern crate typed_builder;
extern crate uom;

pub mod device;
pub mod event;
mod ffi;
pub mod prelude;

use std::{thread, time::Duration};

use async_std::task;
use device::ctx::DeviceContext;
use prelude::*;

async fn main() {
    println!("Hello World!");
    task::sleep(Duration::from_secs(0)).await;
}

#[allow(unused)]
#[ffi_function(namespace = "ffi")]
extern "C" fn start_thread() {
    // spawn the main thread
    thread::spawn(|| task::block_on(main()));
}

#[allow(static_mut_refs, clippy::await_holding_lock, unused)]
#[ffi_function(namespace = "ffi")]
fn supply(context: ffi::FFIData) {
    task::spawn(DeviceContext::instance().replace(context.devices));
}

#[cfg(feature = "build")]
pub mod build {
    use interoptopus::{backend::NamespaceMappings, inventory::Inventory};
    use interoptopus_backend_c::{EnumVariants, Interop, InteropBuilder, NameCase};

    use crate::prelude::*;

    fn __ffi_inventory() -> Inventory {
        let mut builder = InventoryBuilder::new();

        builder = crate::device::__ffi_inventory(builder);
        builder = crate::ffi::__ffi_inventory(builder);

        builder
            .register(function!(crate::supply))
            .register(function!(crate::start_thread))
            .validate()
            .build()
    }

    #[must_use]
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
