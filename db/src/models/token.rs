use chrono::{DateTime, UTC};
use serde_json::Value;
use super::user::User;
use super::utils::{Model, Relation};

#[derive(Debug, Clone)]
pub struct Application {
    pub id: i32,

    pub name: String,
    pub token: String,
}

impl Model for Application {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<Application>> {
        let query = r#"
            SELECT name, token FROM applications
            WHERE id = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let application = rows.iter().next().map(|row| Application {
            id: id,

            name: row.get("name"),
            token: row.get("token"),
        });
        Ok(application)
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: i32,

    pub application: Relation<Application>,
    pub user: Relation<User>,

    pub start: DateTime<UTC>,
    pub end: Option<DateTime<UTC>>,

    pub data: Option<Value>,
}

impl Model for Session {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<Session>> {
        let query = r#"
            SELECT "application_id", "user_id", "start", "end", "data" FROM "sessions"
            WHERE "id" = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let session = rows.iter().next().map(|row| Session {
            id: id,

            application: Relation::new(row.get("application_id")),
            user: Relation::new(row.get("user_id")),

            start: row.get("start"),
            end: row.get("end"),

            data: row.get("data"),
        });
        Ok(session)
    }
}
