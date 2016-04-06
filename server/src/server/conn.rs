use mio::*;
use mio::tcp::TcpStream;
use std::net::SocketAddr;
use super::Server;

pub struct Connection {
    addr: SocketAddr,
    socket: TcpStream,
    token: Token,
}

#[allow(dead_code)]
impl Connection {
    pub fn new(addr: SocketAddr, socket: TcpStream, token: Token) -> Connection {
        Connection {
            addr: addr,
            socket: socket,
            token: token,
        }
    }

    pub fn ready(&mut self, _event_loop: &mut EventLoop<Server>, _events: EventSet) {
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn socket(&self) -> &TcpStream {
        &self.socket
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn register(&self, event_loop: &mut EventLoop<Server>, events: EventSet, poll: PollOpt) {
        event_loop.register(&self.socket, self.token, events, poll).unwrap();
    }
}
