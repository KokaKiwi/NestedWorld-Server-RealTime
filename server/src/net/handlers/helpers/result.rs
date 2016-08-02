use net::conn::Connection;
use net::msg::MessageHeader;
use net::msg::result::{ResultMessage, ResultData};

pub fn handle_with_result<F>(conn: &mut Connection, header: &MessageHeader, f: F) -> bool
    where F: FnOnce(&mut Connection) -> ResultData
{
    let data = f(conn);
    send_result(conn, header, data)
}

pub fn send_result(conn: &mut Connection, header: &MessageHeader, data: ResultData) -> bool {
    let msg = ResultMessage {
        header: MessageHeader {
            id: header.id.clone(),
        },
        data: data,
    };

    match conn.send(msg) {
        Ok(_) => true,
        Err(e) => {
            debug!("Error when sending result: {}", e);
            false
        }
    }
}
