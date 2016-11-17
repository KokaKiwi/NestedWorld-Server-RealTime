//! Describe the internal context of the whole server, containing all global values and links to
//! the server components.
use Config;
use db::Database;
use mioco::sync::Mutex;
use self::error::*;
use self::user::UserStore;
use std::sync::Arc;

pub mod user;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: Database,
    pub users: Arc<Mutex<UserStore>>,
}

impl Context {
    pub fn create(config: Config) -> Result<Context> {
        let db = try!(Database::connect(config.db.clone()));

        Ok(Context {
            config: config,
            db: db,
            users: Arc::new(Mutex::new(UserStore::new())),
        })
    }
}

pub mod error {
    error_chain! {
        links {
            ::db::error::Error, ::db::error::ErrorKind, Database;
        }
    }
}
