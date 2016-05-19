#![feature(pub_restricted, specialization)]

#[macro_use] extern crate chan;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate log;
extern crate mioco;
pub extern crate nestedworld_db;
#[macro_use] extern crate quick_error;
extern crate rmp;
extern crate rustc_serialize;

use mioco::JoinHandle;
use std::net::SocketAddr;

#[macro_use] pub mod net;

#[derive(Debug, Clone)]
pub struct Config {
    /// Server listen address.
    pub listen_addr: SocketAddr,
    /// Secret key used for session handling.
    pub secret: String,
}

pub fn run(config: Config) -> JoinHandle<()> {
    mioco::spawn(move || {
        let net_handle = mioco::spawn(move || net::run(config.clone()));

        // Join all handles in order to keep them running.
        net_handle.join().unwrap();
    })
}
