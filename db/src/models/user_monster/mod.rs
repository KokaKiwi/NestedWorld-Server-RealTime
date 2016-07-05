use super::utils::{Model, Relation};
use super::monster::Monster;
use super::user::User;

#[derive(Debug, Clone)]
pub struct UserMonster {
    pub id: i32,

    pub surname: String,
    pub experience: i32,
    pub level: i32,

    pub hp: i32,

    pub user: Relation<User>,
    pub monster: Relation<Monster>,
}

impl Model for UserMonster {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<UserMonster>> {
        let query = r#"
            SELECT user_id, monster_id, surname, experience, level
            FROM users_monsters
            WHERE id = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let user_monster = rows.iter().next().map(|row| {
            UserMonster {
                id: id,

                surname: row.get("suname"),
                experience: row.get("experience"),
                level: row.get("level"),

                hp: row.get("hp"),

                user: Relation::new(row.get("user_id")),
                monster: Relation::new(row.get("monster_id")),
            }
        });
        Ok(user_monster)
    }
}
