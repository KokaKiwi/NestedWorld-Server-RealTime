#[macro_use] extern crate chan;
#[macro_use] extern crate log;
extern crate mio;
extern crate nestedworld_db;
#[macro_use] extern crate quick_error;

use error::Result;
pub use nestedworld_db as db;
pub use server::config::Config;

pub mod error;
pub mod handle;
mod server;

pub fn start(_config: Config) -> Result<handle::ServerHandle> {
    unimplemented!()
}
