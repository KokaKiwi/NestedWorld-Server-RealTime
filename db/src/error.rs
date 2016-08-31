//! Database errors definition.
error_chain! {
    links {
        self::postgres::Error, self::postgres::ErrorKind, Postgres;
        self::pool::Error, self::pool::ErrorKind, Pool;
    }

    foreign_links {
        ::std::io::Error, Io;
    }

    errors {
    }
}

pub mod postgres {
    error_chain! {
        links {}
        foreign_links {
            ::postgres::error::ConnectError, Connect;
            ::postgres::error::Error, Other;
        }
        errors {}
    }

    impl From<::r2d2_postgres::Error> for Error {
        fn from(err: ::r2d2_postgres::Error) -> Error {
            use r2d2_postgres::Error;

            match err {
                Error::Connect(err) => err.into(),
                Error::Other(err) => err.into(),
            }
        }
    }
}

pub mod pool {
    error_chain! {
        links {}
        foreign_links {
            ::r2d2::InitializationError, Init;
            ::r2d2::GetTimeout, Timeout;
        }
        errors {}
    }
}
