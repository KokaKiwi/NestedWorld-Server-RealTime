use net::msg::{MessagePart};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub id: u32,
    pub name: String,
    pub monster_id: u32,
    pub user_monster_id: u32,
    pub hp: u16,
    pub level: u8,
}

impl MessagePart for Monster {
    fn decode(data: &Value) -> Result<Monster> {
        Ok(Monster {
            id: try!(fields::get(data, "id")),
            name: try!(fields::get(data, "name")),
            monster_id: try!(fields::get(data, "monster_id")),
            user_monster_id: try!(fields::get(data, "user_monster_id")),
            hp: try!(fields::get(data, "hp")),
            level: try!(fields::get(data, "level")),
        })
    }

    fn encode(&self , data: &mut Value) {
        data.set("type", "combat:Monster");
        data.set("id", &self.id);
        data.set("name", &self.name);
        data.set("monster_id", &self.monster_id);
        data.set("user_monster_id", &self.user_monster_id);
        data.set("hp", &self.hp);
        data.set("level", &self.level);
    }
}
