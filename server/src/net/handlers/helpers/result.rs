use net::conn::Connection;
use net::msg::MessageHeader;
use net::msg::result::{ResultMessage, ResultData};

pub fn handle_with_result<F>(conn: &mut Connection, header: &MessageHeader, f: F) -> bool
    where F: FnOnce(&mut Connection) -> ResultData
{
    let res = f(conn);
    let msg = ResultMessage {
        header: MessageHeader {
            id: header.id.clone(),
        },
        data: res,
    };

    match conn.send(msg) {
        Ok(_) => true,
        Err(e) => {
            debug!("Error when sending result: {}", e);
            false
        }
    }
}
