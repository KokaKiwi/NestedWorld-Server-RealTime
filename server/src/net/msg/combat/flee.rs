use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Flee {
    pub header: MessageHeader,
}

impl MessagePart for Flee {
    fn decode(data: &Value) -> Result<Flee> {
        Ok(Flee {
            header: try!(MessageHeader::decode(data)),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:flee");
        self.header.encode(data);
    }
}

impl MessageFull for Flee {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
