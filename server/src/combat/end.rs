use super::result::CombatResult;
use super::math::experience;
use super::math::coefficient;
use db::models::geo::Portal;
use db::models::utils::Model;
use net::conn::Connection;
use chrono::offset::utc::UTC;
use chrono::Duration;
use std::ops::Add;

pub fn end(result: CombatResult, conn: &mut Connection) {
    for monster in result.monsters {
        let mut level = monster.level;
        let mut experience = monster.experience + experience(result.win, result.average_lvl);
        if experience > experience * (experience / 5) + 10 {
            experience = experience - experience * (experience / 5) + 10;
            level += 1;
        }
        let db_conn = conn.ctx.db.get_connection().unwrap();
        db_conn.execute("UPDATE user_monsters SET experience = $1, level = $2 WHERE id = $3", &[&experience, &level, &monster.id]).unwrap();
    }
}


pub fn end_portal(result: CombatResult, conn: &mut Connection, id_portal: i32) {
    if result.win == true {
        let db_conn = conn.ctx.db.get_connection().unwrap();
        let portal = match Portal::get_by_id(&db_conn, id_portal) {
            Ok(Some(portal)) => portal,
            _ => return,
        };
        let mut user_monster = result.monsters[0].clone();
        let monster_type = match user_monster.monster.get_or_fetch(&db_conn) {
            Ok(Some(monster)) => monster.monster_type.clone(),
            _ => return,
        };
        let time = Duration::seconds((3600.0 * coefficient(monster_type, portal.portal_type)) as i64);
        let started = UTC::now();
        let end = started.add(time);
        db_conn.execute("UPDATE portals SET duration = $1, captured = $2, catching_end = $3 WHERE id = $4",
                        &[&time.num_seconds(), &started.to_rfc3339(), &end.to_rfc3339(), &id_portal]).unwrap();
    }
    end(result, conn);
}
