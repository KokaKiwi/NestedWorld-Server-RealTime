use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::states::auth::Authenticated;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Flee {
    pub header: MessageHeader,
    pub auth: Authenticated,
}

impl MessagePart for Flee {
    fn decode(data: &Value) -> Result<Flee> {
        Ok(Flee {
            header: try!(MessageHeader::decode(data)),
            auth: try!(Authenticated::decode(data)),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:flee");
        self.header.encode(data);
        self.auth.encode(data);
    }
}
