use super::utils::Model;

#[derive(Debug, Clone, ToSql, FromSql, PartialEq, Eq)]
#[postgres(name = "attack_type")]
pub enum AttackType {
    #[postgres(name = "attack")]
    Attack,
    #[postgres(name = "attacksp")]
    AttackSp,
    #[postgres(name = "defense")]
    Defense,
    #[postgres(name = "defensesp")]
    DefenseSp
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
                attack_type: row.get("type"),
            }
        });
        Ok(attack)
    }
}
