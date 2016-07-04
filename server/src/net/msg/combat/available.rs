use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result, Error};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::data::user::User;

#[derive(Debug, Clone, PartialEq)]
pub struct Available {
    pub header: MessageHeader,
    pub origin: Origin,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
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

        let origin = try!(fields::get(data, "origin"));
        let origin = match origin {
            "wild-monster" => {
                let monster_id = try!(fields::get(data, "monster_id"));

                Origin::WildMonster {
                    monster_id: monster_id,
                }
            }
            "duel" => {
                let user = try!(fields::get(data, "user"));

                Origin::Duel {
                    user: user,
                }
            }
            _ => return Err(Error::InvalidField("origin", format!("Bad origin type `{}`, should be `wild_monster` or `duel`", origin)))
        };

        Ok(Available {
            header: header,
            origin: origin,
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:available");
        self.header.encode(data);
        match self.origin {
            Origin::WildMonster {
                ref monster_id,
            } => {
                data.set("origin", "wild-monster");
                data.set("monster_id", monster_id);
            }
            Origin::Duel {
                ref user,
            } => {
                data.set("origin", "duel");
                data.set("user", user.value());
            }
        }
    }
}

impl MessageFull for Available {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
