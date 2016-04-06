#[macro_use] extern crate chan;
#[macro_use] extern crate hyper;
#[macro_use] extern crate log;
extern crate mio;
pub extern crate nestedworld_db;
#[macro_use] extern crate quick_error;

use mio::*;
pub use nestedworld_db as db;
pub use server::config::Config;
use server::msg::Message;
use std::io;
use std::thread;

#[macro_use] pub mod utils;
pub mod net;
mod server;

pub struct ServerLoop {
    config: Config,
    event_loop: EventLoop<server::Server>,
}

impl ServerLoop {
    pub fn new(config: Config) -> io::Result<ServerLoop> {
        let event_loop = try!(EventLoop::new());

        Ok(ServerLoop {
            config: config,
            event_loop: event_loop,
        })
    }

    pub fn channel(&self) -> ServerSender {
        ServerSender(self.event_loop.channel())
    }

    pub fn start(self) -> ServerHandle {
        thread::spawn(move || self.run())
    }

    fn run(mut self) {
        use server::Server;

        let mut server = Server::new(self.config).unwrap();
        server.register(&mut self.event_loop);

        self.event_loop.run(&mut server).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct ServerSender(Sender<server::msg::Message>);

impl ServerSender {
    fn send(&self, msg: server::msg::Message) -> io::Result<bool> {
        match self.0.send(msg) {
            Ok(_) => Ok(true),
            Err(NotifyError::Io(err)) => Err(err),
            Err(_) => Ok(false),
        }
    }

    pub fn stop(&self) -> io::Result<bool> {
        self.send(Message::Stop)
    }
}

pub type ServerHandle = thread::JoinHandle<()>;
