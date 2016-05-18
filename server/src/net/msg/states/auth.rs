use rmp::Value;
use net::msg::MessagePart;
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authenticated {
    pub token: String,
}

impl MessagePart for Authenticated {
    fn decode(data: &Value) -> Result<Self> {
        Ok(Authenticated {
            token: try!(fields::get(data, "token")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("token", &self.token);
    }
}
