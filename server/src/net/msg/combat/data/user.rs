use net::msg::MessagePart;
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub pseudo: String,
}

impl MessagePart for User {
    fn decode(data: &Value) -> Result<User> {
        Ok(User {
            id: try!(fields::get(data, "id")),
            pseudo: try!(fields::get(data, "pseudo")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("id", &self.id);
        data.set("pseudo", &self.pseudo);
    }
}
