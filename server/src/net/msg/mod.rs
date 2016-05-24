use rmp::Value;
use self::error::Result;
use self::utils::fields;
use self::utils::rmp::{FromValue, ValueExt};

#[macro_use] pub mod utils;
#[macro_use] mod macros;
pub mod chat;
pub mod combat;
pub mod error;
pub mod result;
pub mod states;

pub trait MessagePart: Sized {
    fn decode(data: &Value) -> Result<Self>;
    fn encode(&self, data: &mut Value);

    fn value(&self) -> Value {
        let mut data = rmp_map![];
        self.encode(&mut data);
        data
    }
}

impl<'a, M: MessagePart> FromValue<'a> for M {
    fn from_value(value: &'a Value) -> Option<M> {
        M::decode(value).ok()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
    /// The message ID, if any.
    pub id: Option<String>,
}

impl MessagePart for MessageHeader {
    fn decode(data: &Value) -> Result<MessageHeader> {
        Ok(MessageHeader {
            id: try!(fields::get(data, "id")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("id", &self.id);
    }
}

message!(Message:
    ref Chat(self::chat::Message),
    type "result" => Result(self::result::ResultMessage),
);
