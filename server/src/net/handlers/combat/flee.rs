use net::conn::Connection;
use net::msg::combat::Flee;
use net::msg::result::ResultData;
use net::handlers::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: Flee) {
    handle_with_result(conn, &msg.header, |conn| {
        match conn.session {
            Some(_) => { return ResultData::ok(None) },
            None => { return ResultData::err("NotAuthenticated", "You are not authenticated on the server", None)}
        }
    })
}
