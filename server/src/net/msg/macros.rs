// Message decoding macro
// Very cryptic macro, but hopefully one day it will be replaced by something better.
macro_rules! __message {
    ([DECLARE ENUM] $(#[$meta:meta])* $name:ident () ($($variant_name:ident($msg_ty:ty)($($variant_meta:meta),*),)*)) => {
        #[derive(Debug, Clone, PartialEq)]
        $(#[$meta])*
        pub enum $name {
            $($(#[$variant_meta])* $variant_name($msg_ty),)*
        }
    };
    ([DECLARE ENUM] $(#[$meta:meta])* $name:ident ($(#[$variant_meta:meta])* type $($ty:expr),* => $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([DECLARE ENUM] $(#[$meta])* $name ($($e)*) ($($r)* $variant_name($msg_ty)($($variant_meta),*),));
    };
    ([DECLARE ENUM] $(#[$meta:meta])* $name:ident ($(#[$variant_meta:meta])* ref $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([DECLARE ENUM] $(#[$meta])* $name ($($e)*) ($($r)* $variant_name($msg_ty)($($variant_meta),*),));
    };

    ([DECODE TEST REFS] $name:ident ($data:expr) ()) => {};
    ([DECODE TEST REFS] $name:ident ($data:expr) ($(#[$meta:meta])* type $($ty:expr),* => $variant_name:ident($msg_ty:ty), $($e:tt)*)) => {
        __message!([DECODE TEST REFS] $name ($data) ($($e)*));
    };
    ([DECODE TEST REFS] $name:ident ($data:expr) ($(#[$meta:meta])* ref $variant_name:ident($msg_ty:ty), $($e:tt)*)) => {
        if let Ok(msg) = $crate::net::msg::MessagePart::decode($data) {
            return Ok($name::$variant_name(msg));
        }

        __message!([DECODE TEST REFS] $name ($data) ($($e)*));
    };
    ([DECODE TEST TYPES] $name:ident ($data:expr, $msg_type:expr) () ($(($($ty:expr),*) => $variant_name:ident,)*)) => {{
        match $msg_type {
            $($($ty)|* => $crate::net::msg::MessagePart::decode($data).map($name::$variant_name),)*
            _ => Err($crate::net::msg::error::Error::InvalidField("type", format!("Unknown message type `{}`", $msg_type))),
        }
    }};
    ([DECODE TEST TYPES] $name:ident ($data:expr, $msg_type:expr) ($(#[$meta:meta])* type $($ty:expr),* => $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([DECODE TEST TYPES] $name ($data, $msg_type) ($($e)*) ($($r)* ($($ty),*) => $variant_name,));
    };
    ([DECODE TEST TYPES] $name:ident ($data:expr, $msg_type:expr) ($(#[$meta:meta])* ref $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([DECODE TEST TYPES] $name ($data, $msg_type) ($($e)*) ($($r)*));
    };

    ([ENCODE MESSAGE] $name:ident ($self_:expr, $data:expr) () ($($variant_name:ident,)*)) => {
        match $self_ {
            $($name::$variant_name(ref msg) => msg.encode($data),)*
        }
    };
    ([ENCODE MESSAGE] $name:ident ($self_:expr, $data:expr) ($(#[$meta:meta])* type $($ty:expr),* => $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([ENCODE MESSAGE] $name ($self_, $data) ($($e)*) ($($r)* $variant_name,));
    };
    ([ENCODE MESSAGE] $name:ident ($self_:expr, $data:expr) ($(#[$meta:meta])* ref $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([ENCODE MESSAGE] $name ($self_, $data) ($($e)*) ($($r)* $variant_name,));
    };

    ([MESSAGE HEADER] $name:ident ($self_:expr) () ($($variant_name:ident,)*)) => {
        match $self_ {
            $($name::$variant_name(ref msg) => msg.header(),)*
        }
    };
    ([MESSAGE HEADER] $name:ident ($self_:expr) ($(#[$meta:meta])* type $($ty:expr),* => $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([MESSAGE HEADER] $name ($self_) ($($e)*) ($($r)* $variant_name,));
    };
    ([MESSAGE HEADER] $name:ident ($self_:expr) ($(#[$meta:meta])* ref $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([MESSAGE HEADER] $name ($self_) ($($e)*) ($($r)* $variant_name,));
    };

    ([MESSAGE HEADER MUT] $name:ident ($self_:expr) () ($($variant_name:ident,)*)) => {
        match $self_ {
            $($name::$variant_name(ref mut msg) => msg.header_mut(),)*
        }
    };
    ([MESSAGE HEADER MUT] $name:ident ($self_:expr) ($(#[$meta:meta])* type $($ty:expr),* => $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([MESSAGE HEADER MUT] $name ($self_) ($($e)*) ($($r)* $variant_name,));
    };
    ([MESSAGE HEADER MUT] $name:ident ($self_:expr) ($(#[$meta:meta])* ref $variant_name:ident($msg_ty:ty), $($e:tt)*) ($($r:tt)*)) => {
        __message!([MESSAGE HEADER MUT] $name ($self_) ($($e)*) ($($r)* $variant_name,));
    };

    ([IMPL MESSAGE] $name:ident ($($e:tt)*)) => {
        impl $crate::net::msg::MessagePart for $name {
            fn decode(data: &$crate::rmp::Value) -> $crate::net::msg::error::Result<$name> {
                __message!([DECODE TEST REFS] $name (data) ($($e)*));

                let msg_type: &str = try!($crate::net::msg::utils::fields::get(data, "type"));
                __message!([DECODE TEST TYPES] $name (data, msg_type) ($($e)*) ())
            }

            fn encode(&self, data: &mut $crate::rmp::Value) {
                __message!([ENCODE MESSAGE] $name (*self, data) ($($e)*) ());
            }
        }

        impl $crate::net::msg::MessageFull for $name {
            fn header(&self) -> &$crate::net::msg::MessageHeader {
                __message!([MESSAGE HEADER] $name (*self) ($($e)*) ())
            }

            fn header_mut(&mut self) -> &mut $crate::net::msg::MessageHeader {
                __message!([MESSAGE HEADER MUT] $name (*self) ($($e)*) ())
            }
         }
    };
}

macro_rules! message {
    ($(#[$meta:meta])* $name:ident {
        $($e:tt)*
    }) => {
        __message!([DECLARE ENUM] $(#[$meta])* $name ($($e)*) ());
        __message!([IMPL MESSAGE] $name ($($e)*));
    };
}
