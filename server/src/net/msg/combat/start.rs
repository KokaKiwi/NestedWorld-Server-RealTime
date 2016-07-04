use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
pub use super::data::start::user::User;
pub use super::data::start::opponent::Opponent;

#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    pub header: MessageHeader,
    pub id: u32,
    pub user: User,
    pub opponent: Opponent,
    pub combat_type: String,
    pub env: String,
    pub first: bool,
}

impl MessagePart for Start {
    fn decode(data: &Value) -> Result<Start> {
        Ok(Start {
            header: try!(MessageHeader::decode(data)),
            id: try!(fields::get(data, "id")),
            user: try!(fields::get(data, "user")),
            opponent: try!(fields::get(data, "opponent")),
            combat_type: try!(fields::get(data, "combat_type")),
            env: try!(fields::get(data, "env")),
            first: try!(fields::get(data, "first")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:start");
        self.header.encode(data);
        data.set("id", &self.id);
        data.set("user", self.user.value());
        data.set("opponent", self.opponent.value());
        data.set("combat_type", &self.combat_type);
        data.set("env", &self.env);
        data.set("first", &self.first);
    }
}

impl MessageFull for Start {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
