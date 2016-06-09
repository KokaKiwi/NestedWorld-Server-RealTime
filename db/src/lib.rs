#![feature(plugin, custom_derive)]
#![plugin(postgres_derive_macros)]

extern crate chrono;
#[macro_use] extern crate log;
#[macro_use] extern crate postgres;
#[macro_use] extern crate quick_error;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_json;

use error::Result;
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
    pub fn connect(db_config: Config) -> Result<Database> {
        use r2d2::Config as PoolConfig;
        use r2d2_postgres::SslMode;

        let pool_config = PoolConfig::default();
        let manager = try!(PostgresConnectionManager::new(db_config.url.as_str(), SslMode::None));
        let pool = try!(Pool::new(pool_config, manager));

        Ok(Database {
            _config: db_config,
            pool: pool,
        })
    }

    pub fn get_connection(&self) -> Result<Connection> {
        self.pool.get().map_err(From::from)
    }

    pub fn get_model<T: Model>(&self, id: i32) -> Result<Option<T>> {
        let conn = try!(self.get_connection());
        T::get_by_id(&conn, id).map_err(From::from)
    }
}
