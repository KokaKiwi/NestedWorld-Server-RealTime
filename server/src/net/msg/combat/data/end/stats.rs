use net::msg::{MessagePart};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub id: u32,
    pub exp: u32,
    pub level: u8
}

impl MessagePart for Stats {
    fn decode(data: &Value) -> Result<Stats> {
        Ok(Stats {
            id: try!(fields::get(data, "id")),
            exp: try!(fields::get(data, "exp")),
            level: try!(fields::get(data, "level")),
        })
    }

    fn encode(&self , data: &mut Value) {
        data.set("id", &self.id);
        data.set("exp", &self.exp);
        data.set("level", &self.level);
    }
}
