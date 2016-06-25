use super::conn::Connection;
use super::msg::Message;

#[macro_use] mod helpers;
mod auth;
mod combat;

pub fn handle(conn: &mut Connection, msg: Message) {
    match msg {
        Message::Authenticate(msg) => auth::handle(conn, msg),
        Message::Combat::SendAttack(msg) => combat::send_attack::handle(conn, msg),
        Message::Combat::Flee(msg) => combat::flee::handle(conn, msg),
        Message::Combat::MonsterKo::Capture(msg) => combat::monster_ko_capture::handle(conn, msg),
        _ => {}
    }
}
