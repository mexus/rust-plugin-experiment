// extern crate libloading as lib;

use common::{load_plugin, PluginInterface};
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
    let plugins: Vec<_> = conf
        .plugins
        .into_iter()
        .map(Some)
        .map(load_plugin)
        .collect::<Result<_, _>>()
        .unwrap();
    for plugin in &plugins {
        println!("Loaded {}", plugin.get_name());
    }
}
