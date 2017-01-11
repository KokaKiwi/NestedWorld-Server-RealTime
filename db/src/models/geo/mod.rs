use super::utils::Model;
use super::commons::ElementType;
use postgis::ewkb::Point;
use chrono::DateTime;
use chrono::UTC;

#[derive(Debug, Clone)]
pub struct Portal {
    pub id: i32,
    pub point: Point,
    pub portal_type: ElementType,
    pub created: DateTime<UTC>,
    pub captured: Option<DateTime<UTC>>,
    pub captured_by: Option<i32>,
    pub umonster_on: Option<i32>,
    pub monster_on: Option<i32>,
    pub duration: Option<u32>,
    pub catching_end: Option<DateTime<UTC>>,
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
                captured_by: row.get("captured_by"),
                umonster_on: row.get("umonster_on"),
                monster_on: row.get("monster_on"),
                duration: row.get("duration"),
                catching_end: row.get("catching_end"),
            }
        });
        Ok(portal)
    }
}
