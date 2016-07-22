use net::conn::Connection;
use net::msg::combat::Message;

pub mod send_attack;
pub mod monster_ko;
pub mod flee;
pub mod ask;

pub fn handle(conn: &mut Connection, msg: Message) {
    match msg {
        Message::SendAttack(msg) => self::send_attack::handle(conn, msg),
        Message::Flee(msg) => self::flee::handle(conn, msg),
        Message::MonsterKo(msg) => self::monster_ko::handle(conn, msg),
        Message::Ask(msg) => self::ask::handle(conn, msg),
        _ => {}
    }
}
