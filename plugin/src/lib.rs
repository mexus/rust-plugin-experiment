use common::PluginInterface;

pub struct SomePlugin;
impl PluginInterface for SomePlugin {
    fn get_name(&self) -> &str {
        "Some plugin"
    }
}

#[no_mangle]
pub fn get_plugin() -> Box<PluginInterface> {
    Box::new(SomePlugin)
}
