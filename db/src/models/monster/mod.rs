use super::utils::Model;

#[derive(Debug, Clone)]
pub enum MonsterType { Water, Fire, Earth, Electric, Plant }

impl MonsterType {
    pub fn from_str(monster_type: &str) -> MonsterType {
        match monster_type {
            "water" => MonsterType::Water,
            "fire" => MonsterType::Fire,
            "earth" => MonsterType::Earth,
            "electric" => MonsterType::Electric,
            "plant" => MonsterType::Plant,
            _ => MonsterType::Fire,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: i32,
    pub name: String,
    pub monster_type: MonsterType,
    pub attack: i32,
    pub hp: i32,
    pub speed: i32,
    pub defense: i32,
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
                monster_type: MonsterType::from_str(&row.get::<_, String>("type")),

                attack: row.get("attack"),
                hp: row.get("hp"),
                speed: row.get("speed"),
                defense: row.get("defense"),
            }
        });
        Ok(monster)
    }
}
