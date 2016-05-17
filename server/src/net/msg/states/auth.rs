use rmp::Value;
use net::msg::MessagePart;
use net::msg::error::Result;
use net::msg::utils::fields::FieldType;

pub struct Authenticated {
    pub token: String,
}

impl MessagePart for Authenticated {
    fn decode(data: &Value) -> Result<Self> {
        Ok(Authenticated {
            token: try!(FieldType::get(data, "token")),
        })
    }

    fn encode(&self, data: &mut Value) {
        self.token.set(data, "token");
    }
}
