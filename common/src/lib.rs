use libc::{RTLD_NODELETE, RTLD_NOW};
use std::ffi::OsStr;

pub trait PluginInterface {
    fn get_name(&self) -> &str;
}

struct SharedLibPlugin {
    lib: Option<libloading::Library>,
    plugin: Option<Box<PluginInterface>>,
}

impl PluginInterface for SharedLibPlugin {
    fn get_name(&self) -> &str {
        self.plugin.as_ref().unwrap().get_name()
    }
}

impl Drop for SharedLibPlugin {
    fn drop(&mut self) {
        self.plugin.take();
        self.lib.take();
    }
}

pub fn load_plugin(path: Option<impl AsRef<OsStr>>) -> libloading::Result<impl PluginInterface> {
    let os_lib = libloading::os::unix::Library::open(path, RTLD_NODELETE | RTLD_NOW)?;
    let lib = libloading::Library::from(os_lib);
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
