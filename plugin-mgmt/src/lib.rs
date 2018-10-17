//! Dynamic plugin management helper.
//!
//! # Allocations
//!
//! By default the library enforces the
//! [`System`](https://doc.rust-lang.org/std/alloc/struct.System.html) allocator to be used by
//! explicitly setting a global allocator to it.
//!
//! If this behaviour is undesired and you know what you're doing, use this library with a feature
//! `no_enforce_system_allocator` activated.

extern crate libloading;

#[cfg(not(feature = "no_enforce_system_allocator"))]
#[global_allocator]
static A: ::std::alloc::System = ::std::alloc::System;

use std::ffi::OsStr;
use std::ops::Deref;

pub struct SharedLibPlugin<T: ?Sized> {
    plugin_interface: *mut T,
    lib: Option<libloading::Library>,
}

impl<T: ?Sized> SharedLibPlugin<T> {
    pub fn from_path(path: impl AsRef<OsStr>) -> libloading::Result<Self> {
        let lib = libloading::Library::new(path)?;
        let plugin: Box<T> = unsafe {
            let func: libloading::Symbol<unsafe fn() -> Box<T>> = lib.get(b"get_plugin")?;
            func()
        };
        let plugin_interface = Box::into_raw(plugin);
        Ok(SharedLibPlugin {
            plugin_interface,
            lib: Some(lib),
        })
    }
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
