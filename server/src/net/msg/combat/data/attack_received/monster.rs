use net::msg::{MessagePart};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub id: u32,
    pub hp: u16,
}

impl MessagePart for Monster {
    fn decode(data: &Value) -> Result<Monster> {
        Ok(Monster {
            id: try!(fields::get(data, "id")),
            hp: try!(fields::get(data, "hp")),
        })
    }

    fn encode(&self , data: &mut Value) {
        data.set("id", &self.id);
        data.set("hp", &self.hp);
    }
}
