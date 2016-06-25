use net::conn::Connection;
use net::msg::combat::monster_ko::Capture;
use net::msg::result::ResultData;
use super::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: Capture) {
    handle_with_result(conn, &msg.header, |conn| {
        ResultData::ok(None);
    })
}
