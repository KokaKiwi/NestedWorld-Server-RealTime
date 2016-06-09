use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Replace {
    pub header: MessageHeader,
    pub user_monster_id: u32,
}

impl MessagePart for Replace {
    fn decode(data: &Value) -> Result<Replace> {
        Ok(Replace {
            header: try!(MessageHeader::decode(data)),
            user_monster_id: try!(fields::get(data, "user_monster_id")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-ko:replace");
        self.header.encode(data);
        data.set("user_monster_id", &self.user_monster_id);
    }
}
