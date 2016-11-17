use chrono::{DateTime, UTC};
use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::{Result, ErrorKind};
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::data::user::User;

#[derive(Debug, Clone, PartialEq)]
pub struct Available {
    pub header: MessageHeader,
    pub origin: Origin,
    pub monsters_max_count: u32,
}

impl MessagePart for Available {
    fn decode(data: &Value) -> Result<Available> {
        let header = try!(MessageHeader::decode(data));

        Ok(Available {
            header: header,
            origin: try!(Origin::decode(data)),
            monsters_max_count: try!(fields::get(data, "monsters_max_count")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "combat:available");
        self.header.encode(data);
        self.origin.encode(data);
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

#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    WildMonster {
        monster_id: u32,
    },
    Duel {
        user: User,
    },
    Portal {
        user: User,
        timeout: DateTime<UTC>,
    },
}

impl MessagePart for Origin {
    fn decode(data: &Value) -> Result<Origin> {
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
            "portal" => {
                let user = try!(fields::get(data, "user"));
                let timeout = try!(fields::get(data, "timeout"));

                Origin::Portal {
                    user: user,
                    timeout: timeout,
                }
            }
            _ => {
                let msg = format!("Bad origin type: `{}`", origin);
                let error = ErrorKind::InvalidField("origin", msg);
                return Err(error.into());
            }
        };
        Ok(origin)
    }

    fn encode(&self, data: &mut Value) {
        match *self {
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
            _ => {}
        }
    }
}
