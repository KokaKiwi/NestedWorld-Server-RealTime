use super::monster::Monster;

pub struct WildMonster {
    pub monster: Monster
}

impl WildMonster {
    pub fn generate(conn: &::postgres::Connection) -> ::postgres::Result<Option<WildMonster>>{

        let query_random = r#"
            SELECT name, type, attack, hp, speed, defense
            FROM monsters
            ORDER BY RANDOM()
            LIMIT 1
        "#;

        let rows = try!(conn.query(query_random, &[]));
        let monster = rows.iter().next().map(|row| {
            WildMonster {
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
