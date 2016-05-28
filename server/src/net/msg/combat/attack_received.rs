use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::data::attack_received::monster::Monster;

#[derive(Debug, Clone, PartialEq)]
pub struct AttackReceived {
    pub header: MessageHeader,
    pub attack: u32,
    pub monster: Monster,
    pub target: Monster,
}

impl MessagePart for AttackReceived {
    fn decode(data: &Value) -> Result<AttackReceived> {
        Ok(AttackReceived {
            header: try!(MessageHeader::decode(data)),
            attack: try!(fields::get(data, "attack")),
            monster: try!(fields::get(data, "monster")),
            target: try!(fields::get(data, "target")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:attack-received");
        self.header.encode(data);
        data.set("attack", &self.attack);
        data.set("monster", self.monster.value());
        data.set("target", self.target.value());
    }
}
