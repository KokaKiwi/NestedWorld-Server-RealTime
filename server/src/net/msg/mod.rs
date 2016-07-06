use rmp::Value;
use self::error::Result;
use self::utils::fields;
use self::utils::rmp::{FromValue, ValueExt};
pub use self::auth::Authenticate;
pub use self::result::ResultMessage;

#[macro_use] pub mod utils;
#[macro_use] mod macros;
pub mod auth;
pub mod chat;
pub mod combat;
pub mod error;
pub mod result;

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

pub trait MessageFull: MessagePart {
    fn header(&self) -> &MessageHeader;
    fn header_mut(&mut self) -> &mut MessageHeader;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
    /// The message ID, if any.
    pub id: Option<String>,
}

impl MessageHeader {
    pub fn new() -> MessageHeader {
        let mut header = MessageHeader {
            id: None,
        };
        header.ensure_id();
        header
    }

    pub fn ensure_id(&mut self) -> String {
        use uuid::Uuid;

        let id = self.id.take().unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self.id = Some(id.clone());
        id
    }
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

message!(Message {
    #[doc = "Chat messages"]
    ref Chat(self::chat::Message),
    #[doc = "Combat messages"]
    ref Combat(self::combat::Message),
    type "authenticate" => Authenticate(Authenticate),
    type "result" => Result(ResultMessage),
});
