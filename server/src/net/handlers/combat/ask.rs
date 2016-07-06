use net::conn::Connection;
use net::msg::combat::Ask;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::result::ResultData;
use net::handlers::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: Ask) {
    handle_with_result(conn, &msg.header, |conn| {
        match conn.session {
            Some(_) => {
                let available =  {
                    header: msg.header,
                    origin: Origin
                };

                match conn.send(ko) {
                    Ok(_) => {}
                    Err(e) => { debug!("Error when sending monsterko: {}", e); }
                }
                return ResultData::ok(None)
            },
            None => { return ResultData::err("NotAuthenticated", "You are not authenticated on the server", None)}
        }
    });
}
