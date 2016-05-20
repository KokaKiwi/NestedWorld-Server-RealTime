use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::states::auth::Authenticated;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::data::attack_received::monster::Monster;

#[derive(Debug, Clone, PartialEq)]
pub struct AttackReceived {
    pub header: MessageHeader,
    pub auth: Authenticated,
    pub attack: u32,
    pub monster: Monster,
    pub target: Monster,
}

impl MessagePart for AttackReceived {
    fn decode(data: &Value) -> Result<AttackReceived> {
        Ok(AttackReceived {
            header: try!(MessageHeader::decode(data)),
            auth: try!(Authenticated::decode(data)),
            attack: try!(fields::get(data, "attack")),
            monster: try!(Monster::decode(data)),
            target: try!(Monster::decode(data)),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:attack-received");
        self.header.encode(data);
        self.auth.encode(data);
        data.set("attack", &self.attack);
        self.monster.encode(data);
        self.target.encode(data);
    }
}
