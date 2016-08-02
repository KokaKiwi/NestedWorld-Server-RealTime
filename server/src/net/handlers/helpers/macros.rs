macro_rules! handler_try {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(value) => value,
            Err(e) => {
                debug!("{}: {}", $msg, e);
                return;
            }
        }
    };

    ($expr:expr) => {
        handler_try!($expr, "Unexpected error")
    };
}
