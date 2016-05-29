use net::msg::{MessagePart, MessageHeader};
use net::msg::error::{Result, Error};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::data::user::User;

#[derive(Debug, Clone, PartialEq)]
pub struct Available {
    pub header: MessageHeader,
    pub data: OriginData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OriginData {
    WildMonster {
        monster_id: u32,
    },
    Duel {
        user: User,
    },
}

impl MessagePart for Available {
    fn decode(data: &Value) -> Result<Available> {
        let header = try!(MessageHeader::decode(data));

        let origin: &str = try!(fields::get(data, "origin"));
        let data = match origin {
            "wild_monster" => {
                let monster_id = try!(fields::get(data, "monster_id"));

                OriginData::WildMonster {
                    monster_id: monster_id,
                }
            }
            "duel" => {
                let user = try!(fields::get(data, "user"));

                OriginData::Duel {
                    user: user,
                }
            }
            _ => return Err(Error::InvalidField("origin", format!("Bad origin type `{}`, should be `wild_monster` or `duel`", origin)))
        };

        Ok(Available {
            header: header,
            data: data,
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:available");
        self.header.encode(data);
        match self.data {
            OriginData::WildMonster {
                ref monster_id,
            } => {
                data.set("origin", "wild_monster");
                data.set("monster_id", monster_id);
            }
            OriginData::Duel {
                ref user,
            } => {
                data.set("origin", "duel");
                data.set("user", user.value());
            }
        }
    }
}
