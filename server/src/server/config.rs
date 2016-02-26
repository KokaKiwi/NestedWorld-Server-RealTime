use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    pub listen: SocketAddr,
}
