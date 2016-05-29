use net::msg::{MessagePart, MessageHeader};
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
}

impl MessagePart for End {
    fn decode(data: &Value) -> Result<End> {
        Ok(End {
            header: try!(MessageHeader::decode(data)),
            status: try!(fields::get(data, "status")),
            stats: try!(fields::get(data, "stats")),
        })

    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:end");
        self.header.encode(data);
        data.set("status", &self.status);
        data.set("stats", self.stats.value());
    }
}
