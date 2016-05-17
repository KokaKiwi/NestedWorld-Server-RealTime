use rmp::Value;
use self::error::Result;
use self::utils::fields::FieldType;

#[macro_use] pub mod utils;
#[macro_use] mod macros;
pub mod chat;
pub mod combat;
pub mod error;
pub mod states;

pub trait MessagePart: Sized {
    fn decode(data: &Value) -> Result<Self>;
    fn encode(&self, data: &mut Value);
}

pub struct MessageHeader {
    /// The message ID, if any.
    pub id: Option<String>,
}

impl MessagePart for MessageHeader {
    fn decode(data: &Value) -> Result<MessageHeader> {
        Ok(MessageHeader {
            id: try!(FieldType::get(data, "id")),
        })
    }

    fn encode(&self, data: &mut Value) {
        self.id.set(data, "id");
    }
}
