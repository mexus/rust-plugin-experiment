//! Dynamic plugin management helper.
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

#[cfg(not(feature = "no_enforce_system_allocator"))]
#[global_allocator]
static A: ::std::alloc::System = ::std::alloc::System;

use std::ffi::OsStr;
use std::ops::Deref;

pub use libloading::Result;

pub struct SharedLibPlugin<T: ?Sized> {
    plugin_interface: *mut T,
    lib: Option<libloading::Library>,
}

pub fn load_plugin<T: ?Sized, F, T1, P: AsRef<OsStr>>(
    path: P,
    symbol: &[u8],
    initial_value: T1,
) -> Result<SharedLibPlugin<T>>
where
    F: Fn(T1) -> Box<T>,
{
    let lib = libloading::Library::new(path)?;
    let plugin: Box<T> = unsafe {
        let func: libloading::Symbol<F> = lib.get(symbol)?;
        func(initial_value)
    };
    let plugin_interface = Box::into_raw(plugin);
    Ok(SharedLibPlugin {
        plugin_interface,
        lib: Some(lib),
    })
}

impl<T: ?Sized> Deref for SharedLibPlugin<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.plugin_interface }
    }
}

impl<T: ?Sized> Drop for SharedLibPlugin<T> {
    fn drop(&mut self) {
        // Ensuring an order in which the fields are dropped.
        unsafe {
            let _ = Box::from_raw(self.plugin_interface);
        }
        let _ = self.lib.take();
    }
}
