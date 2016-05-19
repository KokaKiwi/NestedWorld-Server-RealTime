// Message decoding macro
macro_rules! message {
    ($name:ident:
        $(ref $ref_variant_name:ident($ref_msg_ty:ty),)*
        $(type $($ty:expr),+ => $variant_name:ident($msg_ty:ty),)*
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $($ref_variant_name($ref_msg_ty),)*
            $($variant_name($msg_ty),)*
        }

        impl $crate::net::msg::MessagePart for $name {
            fn decode(data: &$crate::rmp::Value) -> $crate::net::msg::error::Result<$name> {
                use $crate::net::msg::error::Error;
                use $crate::net::msg::utils::fields;

                $({
                    if let Ok(msg) = MessagePart::decode(data) {
                        return Ok($name::$ref_variant_name(msg));
                    }
                })*

                let msg_type: &str = try!(fields::get(data, "type"));
                match msg_type {
                    $($($ty)|* => $crate::net::msg::MessagePart::decode(data).map($name::$variant_name),)*
                    _ => Err(Error::InvalidField("type", format!("Unknown message type `{}`", msg_type))),
                }
            }

            fn encode(&self, data: &mut $crate::rmp::Value) {
                match *self {
                    $($name::$ref_variant_name(ref msg) => msg.encode(data),)*
                    $($name::$variant_name(ref msg) => msg.encode(data),)*
                }
            }
        }
    };
}
