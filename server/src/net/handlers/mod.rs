use super::conn::Connection;
use super::msg::{MessageFull, Message};

#[macro_use] mod helpers;
mod auth;
mod combat;

pub fn handle(conn: &mut Connection, msg: Message) {
    let conversation = msg.header().id.as_ref().and_then(|id| conn.get_conversation(id));
    if let Some(tx) = conversation {
        match tx.send(msg) {
            Ok(_) => {}
            Err(e) => {
                debug!("Conversation callback error: {}", e);
            }
        }
        return;
    }

    match msg {
        Message::Authenticate(msg) => auth::handle(conn, msg),
        Message::Combat(msg) => combat::handle(conn, msg),
        _ => {}
    }
}
