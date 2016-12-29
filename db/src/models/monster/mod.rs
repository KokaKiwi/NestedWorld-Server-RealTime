use super::utils::Model;
use super::commons::ElementType;

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: i32,
    pub name: String,
    pub monster_type: ElementType,
    pub attack: f64,
    pub hp: f64,
    pub speed: f64,
    pub defense: f64,
}

impl Model for Monster {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<Monster>> {
        let query = r#"
            SELECT name, type, attack, hp, speed, defense
            FROM monsters
            WHERE id = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let monster = rows.iter().next().map(|row| {
            Monster {
                id: id,

                name: row.get("name"),
                monster_type: row.get("type"),

                attack: row.get("attack"),
                hp: row.get("hp"),
                speed: row.get("speed"),
                defense: row.get("defense"),
            }
        });
        Ok(monster)
    }
}
