use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct SendAttack {
    pub header: MessageHeader,
    pub target: u32,
    pub attack: u32,
}

impl MessagePart for SendAttack {
    fn decode(data: &Value) -> Result<SendAttack> {
        Ok(SendAttack {
            header: try!(MessageHeader::decode(data)),
            target: try!(fields::get(data, "target")),
            attack: try!(fields::get(data, "attack")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:send-attack");
        self.header.encode(data);
        data.set("target", &self.target);
        data.set("attack", &self.attack);
    }
}
