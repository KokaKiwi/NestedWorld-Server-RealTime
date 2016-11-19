use super::monster::MonsterType;

pub struct WildMonster {
    pub level: u32,
    pub name: String,
    pub monster_type: MonsterType,
    pub attack: f64,
    pub hp: f64,
    pub speed: f64,
    pub defense: f64,
}

impl WildMonster {
    fn generate(conn: &::postgres::Connection, level: u32) -> ::postgres::Result<Option<WildMonster>>{

        let query_random = r#"
            SELECT name, type, attack, hp, speed, defense
            FROM monsters
            ORDER BY RANDOM()
            LIMIT 1
        "#;

        let rows = try!(conn.query(query_random, &[]));
        let monster = rows.iter().next().map(|row| {
            WildMonster {
                level: level,
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
