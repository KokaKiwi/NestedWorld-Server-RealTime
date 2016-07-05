use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
pub use self::capture::Capture;
pub use self::replace::Replace;

pub mod capture;
pub mod replace;

#[derive(Debug, Clone, PartialEq)]
pub struct MonsterKo {
    pub header: MessageHeader,
    pub monster: u32,
}

impl MessagePart for MonsterKo {
    fn decode(data: &Value) -> Result<MonsterKo> {
        Ok(MonsterKo {
            header: try!(MessageHeader::decode(data)),
            monster: try!(fields::get(data, "monster")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:monster-ko");
        self.header.encode(data);
        data.set("monster", &self.monster);
    }
}

impl MessageFull for MonsterKo {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}

message!(Message {
    type "combat:monster-ko" => Base(MonsterKo),
    type "combat:monster-ko:capture" => Capture(Capture),
    type "combat:monster-ko:replace" => Replace(Replace),
});
