use mioco;
use mioco::tcp::TcpListener;
use super::Config;

#[macro_use] pub mod msg;
mod conn;

pub(crate) fn run(config: Config) {
    let listener = TcpListener::bind(&config.listen_addr).unwrap();

    loop {
        let conn = match listener.accept() {
            Ok(conn) => conn,
            Err(e) => {
                error!("Error during accepting socket: {}", e);
                continue;
            }
        };

        let config = config.clone();
        mioco::spawn(move || self::conn::run(config, conn));
    }
}
