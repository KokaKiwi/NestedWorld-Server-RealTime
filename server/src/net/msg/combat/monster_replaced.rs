use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
pub use super::data::Monster;

#[derive(Debug, Clone, PartialEq)]
pub struct MonsterReplaced {
    pub header: MessageHeader,
    pub combat: u32,
    pub monster: Monster,
}

impl MessagePart for MonsterReplaced {
    fn decode(data: &Value) -> Result<MonsterReplaced> {
        Ok(MonsterReplaced {
            header: try!(MessageHeader::decode(data)),
            combat: try!(fields::get(data, "combat")),
            monster: try!(fields::get(data, "monster")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-replaced");
        self.header.encode(data);
        data.set("combat", self.combat);
        data.set("monster", self.monster.value());
    }
}

impl MessageFull for MonsterReplaced {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
