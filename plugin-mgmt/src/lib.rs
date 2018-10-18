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
use std::ops::{Deref, DerefMut};

pub use libloading::Result;

pub struct SharedLibPlugin<T: ?Sized> {
    plugin_interface: *mut T,
    lib: Option<libloading::Library>,
}

pub fn load_plugin<T: ?Sized, P: AsRef<OsStr>>(
    path: P,
    symbol: &[u8],
) -> Result<SharedLibPlugin<T>> {
    let lib = libloading::Library::new(path)?;
    let plugin: Box<T> = unsafe {
        let func: libloading::Symbol<fn() -> Box<T>> = lib.get(symbol)?;
        func()
    };
    let plugin_interface = Box::into_raw(plugin);
    Ok(SharedLibPlugin {
        plugin_interface,
        lib: Some(lib),
    })
}

pub fn load_plugin_arg<T: ?Sized, A, P: AsRef<OsStr>>(
    path: P,
    symbol: &[u8],
    arg: A,
) -> Result<SharedLibPlugin<T>> {
    let lib = libloading::Library::new(path)?;
    let plugin: Box<T> = unsafe {
        let func: libloading::Symbol<fn(A) -> Box<T>> = lib.get(symbol)?;
        func(arg)
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

impl<T: ?Sized> DerefMut for SharedLibPlugin<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.plugin_interface }
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
