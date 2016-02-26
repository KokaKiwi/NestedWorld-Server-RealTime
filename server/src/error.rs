use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            display("I/O error: {}", err)
            from()
        }
    }
}
