//! Describe the internal context of the whole server, containing all global values and links to
//! the server components.
use Config;
use db::Database;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: Database,
}

impl Context {
    pub fn create(config: Config) -> Result<Context, Error> {
        let db = try!(Database::connect(config.db.clone()));

        Ok(Context {
            config: config,
            db: db,
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
