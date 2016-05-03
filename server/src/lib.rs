#[macro_use] extern crate chan;
#[macro_use] extern crate log;
extern crate mioco;
pub extern crate nestedworld_db;
#[macro_use] extern crate quick_error;
extern crate rmp;

use mioco::JoinHandle;
use std::net::SocketAddr;

mod net;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_addr: SocketAddr,
}

pub fn run(config: Config) -> JoinHandle<()> {
    mioco::spawn(move || {
        let net_handle = mioco::spawn(move || net::run(config.clone()));

        // Join all handles in order to keep them running.
        net_handle.join().unwrap();
    })
}
