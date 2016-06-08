use db::Database;
use db::models::token::Session;
use jwt;
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;
use super::{MessagePart, MessageHeader};

const TOKEN_ALG: jwt::Algorithm = jwt::Algorithm::HS512;

#[derive(Debug, Clone, PartialEq)]
pub struct Authenticate {
    pub header: MessageHeader,
    pub token: String,
}

impl Authenticate {
    pub fn session(&self, secret: &str) -> ::std::result::Result<SessionData, jwt::errors::Error> {
        let token = try!(jwt::decode(&self.token, secret.as_bytes(), TOKEN_ALG));
        Ok(token.claims)
    }
}

impl MessagePart for Authenticate {
    fn decode(data: &Value) -> Result<Authenticate> {
        Ok(Authenticate {
            header: try!(MessageHeader::decode(data)),
            token: try!(fields::get(data, "token")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "authenticate");
        self.header.encode(data);
        data.set("token", &self.token);
    }
}

#[derive(Debug, Clone, PartialEq, RustcDecodable, RustcEncodable)]
pub struct SessionData {
    pub session_id: u64,
}

impl SessionData {
    pub fn db(&self, db: &Database) -> ::db::error::Result<Option<Session>> {
        let mut session = try!(db.get_model::<Session>(self.session_id as i32));
        if let Some(ref mut session) = session {
            let conn = try!(db.get_connection());
            try!(session.user.fetch(&conn));
        }
        Ok(session)
    }
}
