#[macro_use] extern crate clap;
extern crate ctrlc;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate nestedworld_server as server;
#[macro_use] extern crate quick_error;
extern crate rustc_serialize;
extern crate toml_config;

#[macro_use] mod utils;
mod config;

use config::Config;
use std::default::Default;

fn main() {
    // Configure log
    log4rs::init_file("conf/log.toml", Default::default()).unwrap();

    // Parse CLI arguments
    let matches = clap_app!(nestedworld_cli =>
        (version: env!("CARGO_PKG_VERSION"))

        (@arg CONFIG_FILE: -c --config-file "Specify a config file to use")
    ).get_matches();

    let config_file = matches.value_of("CONFIG_FILE").unwrap_or("conf/config.toml");
    let config = Config::load(config_file);
}
