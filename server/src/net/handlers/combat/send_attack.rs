use net::conn::Connection;
use net::msg::combat::SendAttack;
use net::msg::result::ResultData;
use net::msg::combat::Message;
use net::handlers::helpers::result::send_result;

pub fn handle(conn: &mut Connection, msg: SendAttack) {
        let session = match conn.session() {
            Some(session) => session,
            None => {send_result(conn, &msg.header, ResultData::err("NotAuthenticated", "You are not authenticated on the server", None)); return}
        };
        let user = session.user.get().expect("No user?");

        let mut combat = match conn.ctx.get_combat(msg.combat) {
            Some(combat) => combat,
            None => {send_result(conn, &msg.header, ResultData::err("InvalidCombat", "Invalid combat ID", None)); return}
        };
        combat.send(user.id as u32, &Message::SendAttack(msg));
}
