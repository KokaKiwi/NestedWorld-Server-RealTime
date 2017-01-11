use super::monster::Monster;
use rand::Rng;

pub struct WildMonster {
    pub monster: Monster
}

impl WildMonster {
    pub fn generate(conn: &::postgres::Connection) -> ::postgres::Result<Option<WildMonster>>{

        let query_random = r#"
            SELECT name, type, attack, hp, speed, defense
            FROM monsters
        "#;

        let rows = try!(conn.query(query_random, &[]));
        let rows: Vec<_> = rows.iter().collect();

        let mut rng = ::rand::thread_rng();
        let row = rng.choose(&rows);

        Ok(row.map(|row| WildMonster {
            monster: Monster {
                id: row.get("id"),
                name: row.get("name"),
                monster_type: row.get("type"),
                attack: row.get("attack"),
                hp: row.get("hp"),
                speed: row.get("speed"),
                defense: row.get("defense"),
            }
        }))
    }
}
