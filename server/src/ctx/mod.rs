//! Describe the internal context of the whole server, containing all global values and links to
//! the server components.
use Config;
use combat::builder::CombatHandle;
use db::Database;
use mioco::sync::Mutex;
use net::conn::Connection;
use self::error::*;
use std::collections::HashMap;
use std::sync::Arc;
use utils::store::Store;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: Database,
    pub users: Arc<Mutex<UserStore>>,
    pub combats: Arc<Mutex<CombatStore>>,
}

impl Context {
    pub fn create(config: Config) -> Result<Context> {
        let db = try!(Database::connect(config.db.clone()));

        Ok(Context {
            config: config,
            db: db,
            users: Arc::new(Mutex::new(UserStore::new())),
            combats: Arc::new(Mutex::new(CombatStore::new())),
        })
    }

    pub fn get_user(&self, id: u32) -> Option<Connection> {
        mutex_lock!(self.users).get(&id).and_then(|conn| conn.try_clone().ok())
    }

    pub fn add_user(&mut self, id: u32, conn: Connection) {
        mutex_lock!(self.users).insert(id, conn);
    }

    pub fn get_combat(&self, id: u32) -> Option<CombatHandle> {
        mutex_lock!(self.combats).get(id as usize).map(Clone::clone)
    }

    pub fn add_combat(&mut self, handle: CombatHandle) -> u32 {
        mutex_lock!(self.combats).insert(handle) as u32
    }

    pub fn remove_combat(&mut self, id: u32) {
        mutex_lock!(self.combats).entry(id as usize).remove();
    }
}

pub type UserStore = HashMap<u32, Connection>;
pub type CombatStore = Store<CombatHandle>;

pub mod error {
    error_chain! {
        links {
            Database(::db::error::Error, ::db::error::ErrorKind);
        }
    }
}
