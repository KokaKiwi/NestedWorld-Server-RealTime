use chrono::{DateTime, NaiveDate, UTC};
use super::utils::Model;

#[derive(Debug, Clone, ToSql, FromSql)]
#[postgres(name = "gender_types")]
pub enum Gender {
    #[postgres(name = "female")]
    Female,
    #[postgres(name = "male")]
    Male,
    #[postgres(name = "other")]
    Other
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,

    pub email: String,
    pub registered_at: DateTime<UTC>,
    pub is_active: bool,

    pub pseudo: String,
    pub city: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub avatar: Option<String>,
    pub background: Option<String>,
}

impl Model for User {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<User>> {
        let query = r#"
            SELECT email, registered_at, is_active, pseudo, city, birth_date, gender, avatar, background
            FROM users
            WHERE id = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let user = rows.iter().next().map(|row| {
            User {
                id: id,

                email: row.get("email"),
                registered_at: row.get("registered_at"),
                is_active: row.get("is_active"),

                pseudo: row.get("pseudo"),
                city: row.get("city"),
                birth_date: row.get("birth_date"),
                gender: row.get("gender"),
                avatar: row.get("avatar"),
                background: row.get("background"),
            }
        });
        Ok(user)
    }
}
