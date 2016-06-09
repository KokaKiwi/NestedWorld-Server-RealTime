use super::conn::Connection;
use super::msg::Message;

#[macro_use] mod helpers;
mod auth;

pub fn handle(conn: &mut Connection, msg: Message) {
    match msg {
        Message::Authenticate(msg) => auth::handle(conn, msg),
        _ => {}
    }
}
