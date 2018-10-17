#[macro_use]
extern crate structopt;

extern crate common;

use common::load_plugin;

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
        v = plugin.get_strings();
    }
    println!("Received a vector of {} elements", v.len());
}
