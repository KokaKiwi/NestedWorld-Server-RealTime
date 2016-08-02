use net::msg::{MessagePart};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::super::monster::Monster;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub monster: Monster,
}

impl MessagePart for User {
    fn decode(data: &Value) -> Result<User> {
        Ok(User {
            monster: try!(fields::get(data, "monster")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("monster", &self.monster.value());
    }
}
