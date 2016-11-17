#![feature(pub_restricted, specialization)]
#![recursion_limit = "1024"]
extern crate chrono;
#[macro_use] extern crate error_chain;
extern crate futures;
extern crate jwt;
#[macro_use] extern crate log;
extern crate mioco;
pub extern crate nestedworld_db as db;
extern crate rand;
extern crate rmp;
extern crate rustc_serialize;
extern crate slab;
extern crate uuid;

use ctx::Context;
use mioco::JoinHandle;
use std::net::SocketAddr;

pub mod utils;
#[macro_use] pub mod net;
pub mod combat;
pub mod ctx;

#[derive(Debug, Clone)]
pub struct Config {
    /// Server listen address.
    pub listen_addr: SocketAddr,
    /// Secret key used for session handling.
    pub secret: String,
    /// Database configuration
    pub db: db::Config,
}

pub fn run(config: Config) -> JoinHandle<()> {
    let ctx = Context::create(config).unwrap();

    mioco::spawn(move || {
        let net_handle = mioco::spawn(move || net::run(ctx.clone()));

        // Join all handles in order to keep them running.
        net_handle.join().unwrap();
    })
}
