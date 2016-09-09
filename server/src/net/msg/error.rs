error_chain! {
    errors {
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
