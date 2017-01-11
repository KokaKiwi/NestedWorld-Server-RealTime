use chrono::*;
use db::models::utils::Model;
use db::models::user_monster::UserMonster;
use db::models::user::User;
use db::models::geo::Portal;
use net::conn::Connection;
use net::msg::portal::Capture;
use net::msg::result::ResultData;
use net::handlers::helpers::result::send_result;
use combat::prepare::get_user;
use combat::math::coefficient;
use combat::prepare::prepare_portal_combat;

pub fn handle(conn: &mut Connection, msg: Capture) {
    debug!("stat to capture a portal");
    let mut msg = msg;
    let uuid = msg.header.ensure_id();

    let mut db_conn = conn.ctx.db.get_connection().unwrap();
    let portal = match Portal::get_by_id(&db_conn, msg.portal_id as i32) {
        Ok(Some(portal)) => portal,
        _ => {
            send_result(conn,
                        &msg.header,
                        ResultData::err("InvalidPortal", "This portal does not exist.", None));
            return
        }
    };
    let mut umonster = match UserMonster::get_by_id(&db_conn, msg.monster as i32) {
        Ok(Some(monster)) => monster,
        _=> {
             send_result(conn,
                         &msg.header,
                         ResultData::err("InvalidMonster", "This monster does not exist", None));
             return;
         }
    };
    let monster = match umonster.monster.get_or_fetch(&db_conn) {
        Ok(Some(monster)) => monster,
        _ => {
            send_result(conn,
                        &msg.header,
                        ResultData::err("internal", "Internal server error", None));
            return;
        }

    };
    let user = match get_user(conn, &mut db_conn) {
        Ok(user) => user,
        _ => {
             send_result(conn,
                         &msg.header,
                         ResultData::err("internal", "internal server error", None));
             return;
         }
    };
    let data = match portal.captured_by {
        Some(user) =>  {
            let pseudo = match User::get_by_id(&db_conn, user) {
                Ok(Some(user)) => user.pseudo,
                _ => {
                    send_result(conn,
                                &msg.header,
                                ResultData::err("internal", "Internal server error", None));
                    return;
                }
            };
            let monster_on = match portal.monster_on {
                Some(monster) => monster,
                _ => {
                    send_result(conn,
                                &msg.header,
                                ResultData::err("internal", "Internal server error", None));
                    return;
                }
            };
            let mut opp_umonster = match UserMonster::get_by_id(&db_conn, monster_on) {
                Ok(Some(monster)) => monster,
                _ => {
                    send_result(conn,
                                &msg.header,
                                ResultData::err("internal", "Internal server error", None));
                    return;
                }
            };
            let opp_monster = match opp_umonster.monster.get_or_fetch(&db_conn) {
                Ok(Some(monster)) => monster,
                _ => {
                    send_result(conn,
                                &msg.header,
                                ResultData::err("internal", "Internal server error", None));
                    return;
                }
            };
            rmp_map![
            "state" => "occupied",
            "owner" => rmp_map!["id" => user, "pseudo" => pseudo],
            "monster" => rmp_map!["id" => 1,
                                  "name" => opp_umonster.surname,
                                  "monster_id" => opp_monster.id,
                                  "user_monster_id" => opp_umonster.id,
                                  "hp" => opp_monster.hp,
                                  "level" => opp_umonster.level]

        ]},
        _ => {
            let duration = (1800.0 * coefficient(portal.portal_type, monster.monster_type.clone())) as i32;
            let now = UTC::now();
            db_conn.execute("UPDATE portals SET captured = $1, captured_by = $2, umonster_on = $3,
            monster_on = $7, duration = $4, catching_end = $5 WHERE id = $6",
            &[&now, &user.id, &umonster.id, &duration, &now.checked_add(Duration::seconds(duration as i64)).unwrap(), &portal.id, &monster.id]).unwrap();
            rmp_map![
                "state" => "vacant",
            ]
        },
    };
    send_result(conn, &msg.header, ResultData::ok(Some(data)));
    match portal.captured_by {
        Some(_user) =>  {
            let opponent_conn = match {
               let opp = portal.captured_by.unwrap();
               let users = conn.ctx.users.lock().unwrap_or_else(|e| e.into_inner());
               users.get(&(opp as u32)).map(|conn| conn.try_clone().unwrap())
           } {
               Some(conn) => conn,
               _ => {
                   send_result(conn,
                               &msg.header,
                               ResultData::err("internal", "Internal server error", None));
                   debug!("no connexion :(");
                   return;
               }
           };
           let monster_on = match portal.monster_on {
               Some(monster) => monster,
               _ => {
                   send_result(conn,
                               &msg.header,
                               ResultData::err("internal", "Internal server error", None));
                   debug!("no monster on :(");
                   return;
               }
           };
           let opp_umonster = match UserMonster::get_by_id(&db_conn, monster_on) {
               Ok(Some(monster)) => monster,
               _ => {
                   send_result(conn,
                               &msg.header,
                               ResultData::err("internal", "Internal server error", None));
                   debug!("bad id !");
                   return;
               }
           };
            prepare_portal_combat(conn, umonster.id, opp_umonster.id, portal.id, Some(opponent_conn), uuid);
        }
        _ => {},
    }
}
