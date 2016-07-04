use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Capture {
    pub header: MessageHeader,
    pub capture: bool,
    pub name: Option<String>,
}

impl MessagePart for Capture {
    fn decode(data: &Value) -> Result<Capture> {
        Ok(Capture {
            header: try!(MessageHeader::decode(data)),
            capture: try!(fields::get(data, "capture")),
            name: try!(fields::get(data, "name")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-ko:replace");
        self.header.encode(data);
        data.set("capture", &self.capture);
        data.set("name", &self.name);
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
