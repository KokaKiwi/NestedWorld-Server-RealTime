use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::states::auth::Authenticated;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MonsterKoReplace {
    pub header: MessageHeader,
    pub auth: Authenticated,
    pub user_monster_id: u32,
}

impl MessagePart for MonsterKoReplace {
    fn decode(data: &Value) -> Result<MonsterKoReplace> {
        Ok(MonsterKoReplace {
            header: try!(MessageHeader::decode(data)),
            auth: try!(Authenticated::decode(data)),
            user_monster_id: try!(fields::get(data, "user_monster_id")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-ko:replace");
        self.header.encode(data);
        self.auth.encode(data);
        data.set("user_monster_id", &self.user_monster_id);
    }
}
