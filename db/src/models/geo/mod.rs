use super::utils::Model;
use super::commons::ElementType;
use postgis::ewkb::Point;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Portal {
    pub id: i32,
    pub point: Point,
    pub portal_type: ElementType,
    pub created: NaiveDate,
    pub captured: Option<NaiveDate>,
    pub duration: Option<u32>,
    pub catching_end: Option<NaiveDate>,
}

impl Model for Portal {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<Portal>> {
        let query = r#"
            SELECT point, type, created, captured, duration, catching_end
            FROM portals
            WHERE id = $1
        "#;
        let rows = try!(conn.query(query, &[&id]));
        let portal = rows.iter().next().map(|row| {
            Portal {
                id: id,

                point: row.get("point"),
                portal_type: row.get("type"),
                created: row.get("created"),
                captured: row.get("captured"),
                duration: row.get("duration"),
                catching_end: row.get("catching_end"),
            }
        });
        Ok(portal)
    }
}
