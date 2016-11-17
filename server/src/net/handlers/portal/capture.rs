use chrono::UTC;
use net::conn::Connection;
use net::msg::portal::Capture;
use net::msg::result::ResultData;
use net::handlers::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: Capture) {
    handle_with_result(conn, &msg.header, |_| {
        let data = rmp_map![
            "state" => "vacant",
            "timeout" => UTC::now(),
        ];
        ResultData::ok(Some(data))
    });
}
