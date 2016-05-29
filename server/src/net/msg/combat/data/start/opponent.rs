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
    pub user: Option<User>,
}

impl MessagePart for Opponent {
    fn decode(data: &Value) -> Result<Opponent> {
        Ok(Opponent {
            monster: try!(fields::get(data, "monster")),
            monsters_count: try!(fields::get(data, "monsters_count")),
            user: try!(fields::get(data, "user")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("monster", self.monster.value());
        data.set("monsters_count", &self.monsters_count);
        if let Some(ref user) = self.user {
            data.set("user", user.value());
        }
    }
}
