use mio::*;
use mio::tcp::TcpListener;
use mio::util::Slab;
use self::config::Config;
use self::conn::Connection;
use self::error::Result;
use self::msg::Message;

pub mod config;
mod conn;
mod error;
pub mod msg;

const SERVER: Token = Token(0);

pub struct Server {
    config: Config,
    listener: TcpListener,
    connections: Slab<Connection>,
}

impl Server {
    pub fn new(config: Config) -> Result<Server> {
        let listener = try!(TcpListener::bind(&config.listen));

        Ok(Server {
            config: config,
            listener: listener,
            connections: Slab::new_starting_at(Token(1), 1024),
        })
    }

    pub fn register(&self, event_loop: &mut EventLoop<Self>) {
        event_loop.register(&self.listener, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();
    }
}

impl Handler for Server {
    type Timeout = ();
    type Message = Message;

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: EventSet) {
        match token {
            SERVER => {
                match self.listener.accept() {
                    Ok(Some((socket, addr))) => {
                        let token = match self.connections.insert_with(|token| Connection::new(addr, socket, token)) {
                            Some(token) => token,
                            None => {
                                error!("Error during accepting a new connection: Can't insert in slab.");
                                return;
                            }
                        };

                        let conn = &self.connections[token];
                        conn.register(event_loop, EventSet::readable(), PollOpt::edge() | PollOpt::oneshot());

                        info!("New connection from {} ({})", self.connections[token].addr(), token.0);
                    }
                    Ok(None) => {
                    }
                    Err(e) => {
                        error!("An error happened when accepting a new connection: {}", e);
                    }
                }
            }
            _ => {
                self.connections[token].ready(event_loop, events);
            }
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
        match msg {
            Message::Stop => {
                event_loop.shutdown();
            }
        }
    }
}
