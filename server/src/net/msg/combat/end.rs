use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
pub use super::data::end::stats::Stats;

#[derive(Debug, Clone, PartialEq)]
pub struct End {
    pub header: MessageHeader,
    pub status: String,
    pub stats: Stats,
    pub combat: u32,
}

impl MessagePart for End {
    fn decode(data: &Value) -> Result<End> {
        Ok(End {
            header: try!(MessageHeader::decode(data)),
            status: try!(fields::get(data, "status")),
            stats: try!(fields::get(data, "stats")),
            combat: try!(fields::get(data, "combat")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:end");
        self.header.encode(data);
        data.set("status", &self.status);
        data.set("stats", self.stats.value());
        data.set ("combat", self.combat);
    }
}

impl MessageFull for End {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
