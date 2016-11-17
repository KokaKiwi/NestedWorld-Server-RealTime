use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Capture {
    pub header: MessageHeader,
    pub portal_id: u32,
    pub monster: u32,
}

impl MessagePart for Capture {
    fn decode(data: &Value) -> Result<Capture> {
        Ok(Capture {
            header: try!(MessageHeader::decode(data)),
            portal_id: try!(fields::get(data, "portal_id")),
            monster: try!(fields::get(data, "monster")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "portal:capture");
        self.header.encode(data);
        data.set("portal_id", &self.portal_id);
        data.set("monster", &self.monster);
    }
}

impl MessageFull for Capture {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
