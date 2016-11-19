use super::monster::MonsterType;
use super::monster::Monster;

pub struct WildMonster {
    pub level: u32,
    pub monster: Monster
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
                monster: Monster {
                    id: row.get("id"),
                    name: row.get("name"),
                    monster_type: row.get("type"),
                    attack: row.get("attack"),
                    hp: row.get("hp"),
                    speed: row.get("speed"),
                    defense: row.get("defense"),
                }
            }
        });
        Ok(monster)
    }
}
