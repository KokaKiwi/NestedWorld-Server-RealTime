use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result};
use net::msg::states::auth::Authenticated;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MonsterKoCapture {
    pub header: MessageHeader,
    pub auth: Authenticated,
    pub capture: String,
    pub name: String,
}

impl MessagePart for MonsterKoCapture {
    fn decode(data: &Value) -> Result<MonsterKoCapture> {
        Ok(MonsterKoCapture {
            header: try!(MessageHeader::decode(data)),
            auth: try!(Authenticated::decode(data)),
            capture: try!(fields::get(data, "capture")),
            name: try!(fields::get(data, "name")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-ko:replace");
        self.header.encode(data);
        self.auth.encode(data);
        data.set("capture", &self.capture);
        data.set("name", &self.name);
    }
}
