#![recursion_limit = "1024"]
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate nestedworld_server as server;
extern crate rustc_serialize;
extern crate toml_config;

#[macro_use] mod utils;
pub mod config;
