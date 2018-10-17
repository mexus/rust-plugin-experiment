extern crate plugin_mgmt;

#[macro_use]
extern crate structopt;

use plugin_mgmt::{load_plugin, PluginInterface};
use std::ffi::OsString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "plugins loader",
    about = "An experiment on plugins loading."
)]
struct Config {
    #[structopt(parse(from_os_str))]
    plugins: Vec<OsString>,
}

fn main() {
    let conf = Config::from_args();
    let plugins = conf
        .plugins
        .iter()
        .map(load_plugin)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut v = Vec::new();
    for plugin in plugins {
        println!("Loaded {}", plugin.get_name());
        v = plugin.get_vector();
    }
    println!("Received a vector of {} elements", v.len());
}
