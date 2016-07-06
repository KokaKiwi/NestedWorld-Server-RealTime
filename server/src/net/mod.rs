use ctx::Context;
use mioco;
use mioco::tcp::TcpListener;

#[macro_use] pub mod msg;
pub mod conn;
pub mod handlers;
pub mod event;

pub(crate) fn run(ctx: Context) {
    let listener = TcpListener::bind(&ctx.config.listen_addr).unwrap();

    loop {
        let conn = match listener.accept() {
            Ok(conn) => conn,
            Err(e) => {
                error!("Error during accepting socket: {}", e);
                continue;
            }
        };

        let ctx = ctx.clone();
        mioco::spawn(move || self::conn::run(ctx, conn));
    }
}
