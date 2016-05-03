use Config;
use mioco::tcp::TcpStream;
use rmp::decode::value::{read_value, Error as MessagePackError};
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            display("I/O error: {}", err)
            from()
        }
        MessagePack(err: MessagePackError) {
            cause(err)
            description(err.description())
            display("MessagePack decoding error: {}", err)
            from()
        }
    }
}

pub fn run(_config: Config, conn: TcpStream) -> Result<(), Error> {
    let mut conn = conn;

    loop {
        let msg = try!(read_value(&mut conn));
        info!("Received message: {:?}", msg);
    }

    Ok(())
}
