use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::states::auth::Authenticated;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::data::start::user::User;
use super::data::start::opponent::Opponent;

#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    pub header: MessageHeader,
    pub auth: Authenticated,
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
            auth: try!(Authenticated::decode(data)),
            id: try!(fields::get(data, "id")),
            user: try!(User::decode(data)),
            opponent: try!(Opponent::decode(data)),
            combat_type: try!(fields::get(data, "combat_type")),
            env: try!(fields::get(data, "env")),
            first: try!(fields::get(data, "first")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:start");
        self.header.encode(data);
        self.auth.encode(data);
        data.set("id", &self.id);
        self.user.encode(data);
        self.opponent.encode(data);
        data.set("combat_type", &self.combat_type);
        data.set("env", &self.env);
        data.set("first", &self.first);
    }
}
