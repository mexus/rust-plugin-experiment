extern crate libloading;

#[global_allocator]
static A: ::std::alloc::System = ::std::alloc::System;

use std::ffi::OsStr;

pub trait PluginInterface {
    fn get_name(&self) -> &str;
    fn get_vector(&self) -> Vec<u8>;
}

struct SharedLibPlugin {
    plugin: Option<Box<PluginInterface>>,
    lib: Option<libloading::Library>,
}

impl PluginInterface for SharedLibPlugin {
    fn get_name(&self) -> &str {
        self.plugin.as_ref().unwrap().get_name()
    }

    fn get_vector(&self) -> Vec<u8> {
        self.plugin.as_ref().unwrap().get_vector()
    }
}

impl Drop for SharedLibPlugin {
    fn drop(&mut self) {
        // Ensuring an order in which the fields are dropped.
        self.plugin.take();
        self.lib.take();
    }
}

pub fn load_plugin(path: impl AsRef<OsStr>) -> libloading::Result<impl PluginInterface> {
    let lib = libloading::Library::new(path)?;
    let plugin: Box<PluginInterface> = unsafe {
        let func: libloading::Symbol<unsafe fn() -> Box<PluginInterface>> =
            lib.get(b"get_plugin")?;
        func()
    };
    Ok(SharedLibPlugin {
        lib: Some(lib),
        plugin: Some(plugin),
    })
}
