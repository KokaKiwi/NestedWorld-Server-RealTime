use super::utils::Model;

#[derive(Debug, Clone)]
pub enum AttackType { Attack, AttackSp, Defense, DefenseSp}

impl AttackType {
    pub fn from_str(attack_type: &str) -> AttackType {
        match attack_type {
            "attack" => AttackType::Attack,
            "attacksp" => AttackType::AttackSp,
            "defense" => AttackType::Defense,
            "defensesp" => AttackType::DefenseSp,
            _ => AttackType::Attack,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attack {
    pub id: i32,
    pub name: String,
    pub attack_type: AttackType,
}

impl Model for Attack {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<Attack>> {
        let query = r#"
            SELECT name, type
            FROM attacks
            WHERE id = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let attack = rows.iter().next().map(|row| {
            Attack {
                id: id,

                name: row.get("name"),
                attack_type: AttackType::from_str(&row.get::<_, String>("type")),
            }
        });
        Ok(attack)
    }
}
