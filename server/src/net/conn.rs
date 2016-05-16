use Config;
use mioco::tcp::TcpStream;
use rmp::decode::value::read_value;

pub fn run(_config: Config, conn: TcpStream) {
    let mut conn = conn;

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
        info!("Received message: {}", msg);
    }
}
