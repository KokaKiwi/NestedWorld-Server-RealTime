#![feature(pub_restricted, specialization)]

#[macro_use] extern crate chan;
extern crate jwt;
#[macro_use] extern crate log;
extern crate mioco;
pub extern crate nestedworld_db as db;
#[macro_use] extern crate quick_error;
extern crate rmp;
extern crate rustc_serialize;

use ctx::Context;
use mioco::JoinHandle;
use std::net::SocketAddr;

#[macro_use] pub mod net;
mod ctx;

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
