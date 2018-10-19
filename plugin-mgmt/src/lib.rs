//! Dynamic plugin management helper. Experiments over dynamic plugins in Rust.
//!
//! To see the example in action (on linux):
//!
//! ```sh
//! % cd $(git rev-parse --show-toplevel)
//! % cargo build
//! % ./target/debug/example-application ./target/debug/libplugin_example.so
//! ```
//!
//! # Allocations
//!
//! By default the library enforces the
//! [`System`](https://doc.rust-lang.org/std/alloc/struct.System.html) allocator to be used by
//! setting it explicitly via
//! [`#[global_allocator]`](https://doc.rust-lang.org/std/alloc/index.html#the-global_allocator-attribute).
//!
//! If this behaviour is undesired and you know what you're doing, use this library with a feature
//! `no_enforce_system_allocator` activated.

extern crate libloading;

#[macro_use]
extern crate rental;

#[cfg(not(feature = "no_enforce_system_allocator"))]
#[global_allocator]
static A: ::std::alloc::System = ::std::alloc::System;

mod plug_interface;
mod renting;

use libloading::Library;
pub use libloading::Result;
pub use renting::SharedLibPlugin;
use std::ffi::OsStr;

pub fn load_plugin<T: ?Sized, P: AsRef<OsStr>>(
    path: P,
    symbol: &[u8],
) -> Result<SharedLibPlugin<T>> {
    let lib = Library::new(path)?;
    SharedLibPlugin::load_plugin(lib, symbol)
}

pub fn load_plugin_arg<T: ?Sized, Arg, P: AsRef<OsStr>>(
    path: P,
    symbol: &[u8],
    argument: Arg,
) -> Result<SharedLibPlugin<T>> {
    let lib = Library::new(path)?;
    SharedLibPlugin::load_plugin_arg(lib, symbol, argument)
}
