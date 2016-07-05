use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
pub use super::data::user::User;

#[derive(Debug, Clone, PartialEq)]
pub struct Ask {
    pub header: MessageHeader,
    pub opponent: String,
}

impl MessagePart for Ask {
    fn decode(data: &Value) -> Result<Ask> {
        Ok(Ask {
            header: try!(MessageHeader::decode(data)),
            opponent: try!(fields::get(data, "opponent")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:ask");
        self.header.encode(data);
        data.set("opponent", &self.opponent);
    }
}

impl MessageFull for Ask {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
