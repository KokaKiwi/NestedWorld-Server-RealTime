#![feature(proc_macro)]
#![recursion_limit = "1024"]
extern crate chrono;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate postgis;
#[macro_use] extern crate postgres;
#[macro_use] extern crate postgres_derive;
extern crate rand;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_json;

use models::utils::Model;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

pub mod error;
pub mod models;

#[derive(Debug, Clone)]
pub struct Config {
    pub url: String,
}

pub type Connection = PooledConnection<PostgresConnectionManager>;

#[derive(Clone)]
pub struct Database {
    _config: Config,
    pool: Pool<PostgresConnectionManager>,
}

impl Database {
    pub fn connect(db_config: Config) -> error::Result<Database> {
        use r2d2::Config as PoolConfig;
        use r2d2_postgres::TlsMode;

        let pool_config = PoolConfig::default();
        let manager = try!(PostgresConnectionManager::new(db_config.url.as_str(), TlsMode::None)
                            .map_err(error::postgres::Error::from));
        let pool = try!(Pool::new(pool_config, manager)
                            .map_err(error::pool::Error::from));

        Ok(Database {
            _config: db_config,
            pool: pool,
        })
    }

    pub fn get_connection(&self) -> error::Result<Connection> {
        self.pool.get().map_err(error::pool::Error::from).map_err(From::from)
    }

    pub fn get_model<T: Model>(&self, id: i32) -> error::Result<Option<T>> {
        let conn = try!(self.get_connection());
        T::get_by_id(&conn, id).map_err(error::postgres::Error::from).map_err(From::from)
    }
}
