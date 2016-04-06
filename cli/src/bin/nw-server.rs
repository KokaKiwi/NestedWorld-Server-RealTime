#[macro_use] extern crate clap;
extern crate log4rs;
extern crate nestedworld_cli as cli;
extern crate nestedworld_server as server;

use cli::config::Config;
use server::{Config as ServerConfig, ServerLoop};

fn main() {
    // Configure log
    log4rs::init_file("conf/log.toml", Default::default()).unwrap();

    // Parse arguments
    let matches = clap_app!(nw_server =>
        (version: env!("CARGO_PKG_VERSION"))

        (@arg CONFIG_FILE: -c --config-file "Specify a config file to use")
    ).get_matches();

    let config_file = matches.value_of("CONFIG_FILE").unwrap_or("conf/config.toml");
    let config = Config::load(config_file);

    // Run server
    let server_config = ServerConfig {
        listen: config.server.listen(),
    };
    let server_loop = ServerLoop::new(server_config).unwrap();
    let handle = server_loop.start();

    println!("Server started at {}. Press Ctrl+C to stop.", config.server.listen());
    handle.join().unwrap();
}
