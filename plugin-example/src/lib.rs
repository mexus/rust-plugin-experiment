extern crate plugin_mgmt;

use plugin_mgmt::PluginInterface;

pub struct SomePlugin;
impl PluginInterface for SomePlugin {
    fn get_name(&self) -> &str {
        "Some plugin"
    }

    fn get_vector(&self) -> Vec<u8> {
        vec![100; 1024 * 1024]
    }
}

#[no_mangle]
pub fn get_plugin() -> Box<PluginInterface> {
    Box::new(SomePlugin)
}
