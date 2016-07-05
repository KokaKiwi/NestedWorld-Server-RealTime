use net::msg::MessagePart;
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub pseudo: String,
}

impl MessagePart for User {
    fn decode(data: &Value) -> Result<User> {
        Ok(User {
            pseudo: try!(fields::get(data, "pseudo")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("pseudo", &self.pseudo);
    }
}
