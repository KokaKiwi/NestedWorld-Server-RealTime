//! Database errors definition.
pub type Result<T> = ::std::result::Result<T, Error>;

macro_rules! impl_sub_error {
    ($err_ty:ident::$variant:ident($ty:ty)) => {
        impl From<$ty> for $err_ty {
            fn from(err: $ty) -> $err_ty {
                $err_ty::$variant(err.into())
            }
        }
    };
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: ::std::io::Error) {
            cause(err)
            description(err.description())
            display("I/O error: {}", err)
            from()
        }
        Postgres(err: PostgresError) {
            cause(err)
            description(err.description())
            display("PostgreSQL driver error: {}", err)
            from()
        }
        Pool(err: PoolError) {
            cause(err)
            description(err.description())
            display("Connection pool error: {}", err)
            from()
        }
    }
}

impl_sub_error!(Error::Postgres(::postgres::error::ConnectError));
impl_sub_error!(Error::Postgres(::postgres::error::Error));
impl_sub_error!(Error::Postgres(::r2d2_postgres::Error));

impl_sub_error!(Error::Pool(::r2d2::InitializationError));
impl_sub_error!(Error::Pool(::r2d2::GetTimeout));

quick_error! {
    #[derive(Debug)]
    pub enum PostgresError {
        ConnectError(err: ::postgres::error::ConnectError) {
            cause(err)
            description(err.description())
            display("{}", err)
            from()
        }
        Other(err: ::postgres::error::Error) {
            cause(err)
            description(err.description())
            display("{}", err)
            from()
        }
    }
}

impl From<::r2d2_postgres::Error> for PostgresError {
    fn from(err: ::r2d2_postgres::Error) -> PostgresError {
        use r2d2_postgres::Error;

        match err {
            Error::Connect(err) => From::from(err),
            Error::Other(err) => From::from(err),
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum PoolError {
        Init(err: ::r2d2::InitializationError) {
            cause(err)
            description(err.description())
            display("{}", err)
            from()
        }
        Timeout(err: ::r2d2::GetTimeout) {
            cause(err)
            description(err.description())
            display("{}", err)
            from()
        }
    }
}
