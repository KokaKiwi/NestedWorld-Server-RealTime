//! Describe the internal context of the whole server, containing all global values and links to
//! the server components.
use Config;
use combat::store::CombatStore;
use db::Database;
use mioco::sync::Mutex;
use self::user::UserStore;
use std::sync::Arc;

pub mod user;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: Database,
    pub combats: Arc<Mutex<CombatStore>>,
    pub users: Arc<Mutex<UserStore>>,
}

impl Context {
    pub fn create(config: Config) -> Result<Context, Error> {
        let db = try!(Database::connect(config.db.clone()));

        Ok(Context {
            config: config,
            db: db.clone(),
            combats: CombatStore::new(db.clone()),
            users: Arc::new(Mutex::new(UserStore::new())),
        })
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Database(err: ::db::error::Error) {
            cause(err)
            description(err.description())
            display("Database error: {}", err)
            from()
        }
    }
}
