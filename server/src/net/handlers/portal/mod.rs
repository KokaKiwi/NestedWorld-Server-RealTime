use net::conn::Connection;
use net::msg::portal::Message;

pub mod capture;

pub fn handle(conn: &mut Connection, msg: Message) {
    match msg {
        Message::Capture(msg) => self::capture::handle(conn, msg),
    }
}
