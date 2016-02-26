#[macro_use] extern crate log;
#[macro_use] extern crate postgres;
#[macro_use] extern crate quick_error;
extern crate r2d2;
extern crate r2d2_postgres;

use error::Result;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

#[macro_use] mod utils;
pub mod error;

pub struct Database {
    _config: Config,
    pool: Pool<PostgresConnectionManager>,
}

impl Database {
    pub fn connect(db_config: Config) -> Result<Database> {
        use error::{PoolError, PostgresError};
        use r2d2::Config as PoolConfig;
        use r2d2_postgres::SslMode;

        let pool_config = PoolConfig::default();
        let manager = try!(PostgresConnectionManager::new(db_config.url.as_str(), SslMode::None)
                                                    .map_err::<PostgresError, _>(From::from));
        let pool = try!(Pool::new(pool_config, manager).map_err::<PoolError, _>(From::from));

        Ok(Database {
            _config: db_config,
            pool: pool,
        })
    }

    pub fn get_connection(&self) -> Result<PooledConnection<PostgresConnectionManager>> {
        use error::PoolError;
        self.pool.get().map_err::<PoolError, _>(From::from).map_err(From::from)
    }
}

pub struct Config {
    pub url: String,
}
