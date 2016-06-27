use ctx::Context;
use db::models::token::Session;
use mioco::tcp::TcpStream;
use mioco;
use super::msg::{Message, MessagePart};
use rmp::decode::value::read_value;
use rmp::encode::value::Error as EncodeError;
use super::handlers;
use super::event;

#[derive(Clone)]
pub struct Connection {
    pub ctx: Context,
    pub stream: TcpStream,
    pub session: Option<Session>,
}

impl Connection {
    pub fn new(ctx: Context, stream: TcpStream) -> Connection {
        Connection {
            ctx: ctx,
            stream: stream,
            session: None,
        }
    }

    pub fn send<M: MessagePart>(&mut self, msg: M) -> Result<(), EncodeError> {
        use rmp::encode::value::write_value;
        write_value(&mut self.stream, &msg.value())
    }
}

pub fn run(ctx: Context, conn: TcpStream) {
    let mut conn = Connection::new(ctx, conn);

    debug!("Got connection!");
    mioco::spawn(move || read_and_decode(conn.clone()));
    mioco::spawn(move || event::send_random_combat(conn.clone()));
}

pub fn read_and_decode(conn: &mut Connection) {
    loop {
        let msg = match read_value(&mut conn.stream) {
            Ok(msg) => msg,
            Err(e) => {
                // Error during reading value, we just handle this silently by closing the
                // connection.
                debug!("Error reading MessagePack value: {}", e);
                break;
            }
        };

        let msg = match Message::decode(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                debug!("Received an invalid message: {}", e);
                continue;
            }
        };
        handlers::handle(conn, msg);
    }
}
