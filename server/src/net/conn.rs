use ctx::Context;
use db::models::token::Session;
use mioco::tcp::TcpStream;
use mioco;
use super::msg::{Message, MessagePart};
use rmp::decode::value::read_value;
use rmp::encode::value::Error as EncodeError;
use super::handlers;
use super::event;

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

    pub fn try_clone(&self) -> ::std::io::Result<Connection> {
        Ok(Connection {
          ctx: self.ctx.clone(),
          stream: try!(self.stream.try_clone()),
          session: self.session.clone(),
        })
    }
}

pub fn run(ctx: Context, conn: TcpStream) {
    let conn = Connection::new(ctx, conn);

    debug!("Got connection!");

    match conn.try_clone() {
        Ok(cloned_conn) => {
            let mut value = cloned_conn;
            mioco::spawn(move || read_and_decode(&mut value));
        },
        Err(err) => {
            debug!("Error when trying to clone TcpStream connection : {}", err);
        },
    }

    match conn.try_clone() {
        Ok(cloned_conn) => {
            let mut value = cloned_conn;
            mioco::spawn(move || event::send_random_combat(&mut value));
        },
        Err(err) => {
            debug!("Error when trying to clone TcpStream connection : {}", err);
        },
    }
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
