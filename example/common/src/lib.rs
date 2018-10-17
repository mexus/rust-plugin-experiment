extern crate plugin_mgmt;

pub trait CommonTrait {
    fn get_name(&self) -> &str;
    fn get_strings(&self) -> Vec<String>;
}

pub type ExtPlugin = plugin_mgmt::SharedLibPlugin<CommonTrait>;
