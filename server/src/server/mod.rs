use self::config::Config;
use self::error::Result;
use mio::{EventLoop, Handler};
use mio::tcp::TcpListener;
use self::msg::Message;

pub mod config;
pub mod error;
pub mod msg;

pub struct Server {
    config: Config,
    listener: TcpListener,
}

impl Server {
    pub fn new(config: Config) -> Result<Server> {
        let listener = try!(TcpListener::bind(&config.listen));

        Ok(Server {
            config: config,
            listener: listener,
        })
    }
}

impl Handler for Server {
    type Timeout = ();
    type Message = Message;

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
        match msg {
            Message::Stop => {
                event_loop.shutdown();
            }
        }
    }
}
