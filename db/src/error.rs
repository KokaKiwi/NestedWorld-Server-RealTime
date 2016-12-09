//! Database errors definition.
error_chain! {
    links {
        Postgres(self::postgres::Error, self::postgres::ErrorKind);
        Pool(self::pool::Error, self::pool::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
    }
}

pub mod postgres {
    error_chain! {
        links {}
        foreign_links {
            Connect(::postgres::error::ConnectError);
            Other(::postgres::error::Error);
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
            Init(::r2d2::InitializationError);
            Timeout(::r2d2::GetTimeout);
        }
        errors {}
    }
}
