pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        MissingField(path: &'static str) {
            description("Missing message field")
            display("Missing message field: {}", path)
        }
        InvalidField(path: &'static str, msg: String) {
            description("Invalid message field")
            display("Invalid message field: {} (at `{}`)", msg, path)
        }
    }
}
