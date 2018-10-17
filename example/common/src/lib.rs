extern crate plugin_mgmt;

use std::ffi::OsStr;

pub trait CommonTrait {
    fn get_name(&self) -> &str;
    fn get_strings(&self) -> Vec<String>;
}

pub fn load_plugin(path: impl AsRef<OsStr>) -> plugin_mgmt::Result<ExtPlugin> {
    plugin_mgmt::load_plugin::<CommonTrait, fn(_) -> Box<_>, _, _>(path, b"get_plugin", ())
}

pub type ExtPlugin = plugin_mgmt::SharedLibPlugin<CommonTrait>;
