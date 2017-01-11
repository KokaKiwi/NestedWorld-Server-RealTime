use net::conn::Connection;
use net::msg::combat::SendAttack;
use net::msg::result::ResultData;
use net::handlers::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: SendAttack) {
    handle_with_result(conn, &msg.header.clone(), |conn| {
        use net::msg::combat::Message;

        let session = match conn.session() {
            Some(session) => session,
            None => return ResultData::err("NotAuthenticated",
                                           "You are not authenticated", None),
        };
        let user = session.user.get().expect("No user?");

        let mut combat = match conn.ctx.get_combat(msg.combat) {
            Some(combat) => combat,
            None => return ResultData::err("InvalidCombat", "Invalid combat ID", None),
        };
        combat.send(user.id as u32, &Message::SendAttack(msg));

        ResultData::ok(None)
    });
}
