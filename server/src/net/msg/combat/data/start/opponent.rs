use net::msg::{MessagePart};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::super::user::User;
use super::super::monster::Monster;

#[derive(Debug, Clone, PartialEq)]
pub struct Opponent {
    pub monster: Monster,
    pub monsters_count: u8,
    pub user: User,
}

impl MessagePart for Opponent {
    fn decode(data: &Value) -> Result<Opponent> {
        Ok(Opponent {
            monster: try!(Monster::decode(data)),
            monsters_count: try!(fields::get(data, "monsters_count")),
            user: try!(User::decode(data)),
        })
    }

    fn encode(&self, data: &mut Value) {
        self.monster.encode(data);
        self.user.encode(data);
        data.set("monsters_count", &self.monsters_count);
    }
}
