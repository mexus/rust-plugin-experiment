extern crate common;

use common::CommonTrait;

pub struct SomePlugin;
impl CommonTrait for SomePlugin {
    fn get_name(&self) -> &str {
        "Some plugin"
    }

    fn get_strings(&self) -> Vec<String> {
        vec!["Ahahaha".into(); 1024]
    }
}

#[no_mangle]
pub fn get_plugin() -> Box<CommonTrait> {
    Box::new(SomePlugin)
}
