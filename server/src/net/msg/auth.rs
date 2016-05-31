use super::{MessagePart, MessageHeader};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Authenticate {
    pub header: MessageHeader,
    pub token: String,
}

impl MessagePart for Authenticate {
    fn decode(data: &Value) -> Result<Authenticate> {
        Ok(Authenticate {
            header: try!(MessageHeader::decode(data)),
            token: try!(fields::get(data, "token")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "authenticate");
        self.header.encode(data);
        data.set("token", &self.token);
    }
}
