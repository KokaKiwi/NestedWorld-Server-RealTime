use net::msg::{MessagePart};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
}


impl MessagePart for User {
    fn decode(data: &Value) -> Result<User> {
        Ok(User {
            id: try!(fields::get(data, "id")),
            name: try!(fields::get(data, "name")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:User");
        data.set("id", &self.name);
        data.set("user", &self.id);
    }
}
