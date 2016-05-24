use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::states::auth::Authenticated;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MonsterKo {
    pub header: MessageHeader,
    pub monster: u32,
}

impl MessagePart for MonsterKo {
    fn decode(data: &Value) -> Result<MonsterKo> {
        Ok(MonsterKo {
            header: try!(MessageHeader::decode(data)),
            monster: try!(fields::get(data, "monster")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-ko");
        self.header.encode(data);
        data.set("monster", &self.monster);
    }
}
