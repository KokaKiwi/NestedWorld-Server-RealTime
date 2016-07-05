use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
pub use super::data::attack_received::monster::Monster;

#[derive(Debug, Clone, PartialEq)]
pub struct AttackReceived {
    pub header: MessageHeader,
    pub attack: u32,
    pub monster: Monster,
    pub target: Monster,
    pub combat: u32,
}

impl MessagePart for AttackReceived {
    fn decode(data: &Value) -> Result<AttackReceived> {
        Ok(AttackReceived {
            header: try!(MessageHeader::decode(data)),
            attack: try!(fields::get(data, "attack")),
            monster: try!(fields::get(data, "monster")),
            target: try!(fields::get(data, "target")),
            combat: try!(fields::get(data, "combat")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:attack-received");
        self.header.encode(data);
        data.set("attack", &self.attack);
        data.set("monster", self.monster.value());
        data.set("target", self.target.value());
        data.set("combat", self.combat)
    }
}

impl MessageFull for AttackReceived {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
