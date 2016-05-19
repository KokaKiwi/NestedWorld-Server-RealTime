use Config;
use mioco::tcp::TcpStream;
use super::msg::{Message, MessagePart};
use rmp::decode::value::read_value;

pub fn run(_config: Config, conn: TcpStream) {
    let mut conn = conn;

    debug!("Got connection!");
    loop {
        let msg = match read_value(&mut conn) {
            Ok(msg) => msg,
            Err(e) => {
                // Error during reading value, we just handle this silently by closing the
                // connection.
                debug!("Error reading MessagePack value: {}", e);
                break;
            }
        };

        let msg = match Message::decode(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                debug!("Received an invalid message: {}", e);
                continue;
            }
        };

        println!("{:?}", msg);
    }
}
