use net::conn::Connection;
use db::models::token::Session;
use db::models::user_monster::UserMonster;
use db::models::monster::Monster;
use db::models::user::User;
use db::models::utils::Model;
mod db {
    pub use db::Connection;
}
use super::builder;
use super::end;

//add user monster to th builder and return the average level66

pub fn get_user(conn: &mut Connection, db_conn: &mut db::Connection) -> Result<User, String> {
    let session : Session = match conn.session() {
        Some(session) => session,
        None => return Err("User is not connected".into()),
    };
    let id : i32 = match session.user.get() {
        Some(user) => user.id,
        _ => return Err("Internal server error".into()),
    };
    let user = match User::get_by_id(&db_conn, id) {
        Ok(Some(user)) => user,
        _ => return Err("User doesn't exist".into()),
    };
    return Ok(user);
}

pub fn add_user_monster(db_conn: &mut db::Connection, monsters: &[i32], builder: &mut builder::CombatBuilder) -> Result<u32, String> {
    let mut average_level : u32 = 0;
    for monster in monsters {
        let user_monster = match UserMonster::get_by_id(&db_conn, *monster) {
            Ok(Some(monster)) => monster,
            _ => return Err("User not sended the good id's !".into()),
        };
        builder.add_user_monster(&user_monster);
        average_level += user_monster.level as u32;
    }
    Ok(average_level / monsters.len() as u32)
}

pub fn add_opponent_monster(db_conn: &mut db::Connection, monsters: &[i32], builder: &mut builder::CombatBuilder) {
    for monster in monsters {
        match UserMonster::get_by_id(db_conn, *monster) {
            Ok(Some(mut monster)) => {
                match monster.monster.get_or_fetch(db_conn) {
                    Ok(Some(opp_monster)) => {
                        builder.add_opponent_monster (
                            builder::Monster {
                                 monster: opp_monster.clone(),
                                 name: monster.surname,
                                 level: monster.level as u32
                             });
                         },
                    _ => {}
                }
            },
            _ => {},
        };
    }
}

pub fn prepare_wild_combat(conn: &mut Connection, monsters: &[i32], ai_monster: i32) {

    let mut db_conn = conn.ctx.db.get_connection().unwrap();

    let user = match get_user(conn, &mut db_conn) {
        Ok(user) => user,
        Err(e) => return
    };

    if monsters.is_empty() {
        return;
    }

    // Start builder

    let user_infos = builder::UserInfos {user: user, conn: match conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    }};

    let mut builder : builder::CombatBuilder = builder::CombatBuilder::new(user_infos,
                                            builder::OpponentType::AI, "wild", "city");

    //add user monsters in the builder and compute average level
    let average_level = match add_user_monster(&mut db_conn, monsters, &mut builder) {
        Ok(level) => level,
        _ => return,
    };

    //add IA monsters in the builder
    let opp_db_monster = match Monster::get_by_id(&mut db_conn, ai_monster) {
        Ok(Some(monster)) => monster,
        _ => return,
    };

    let monster_name = opp_db_monster.name.clone();
    let opp_monster = builder::Monster {
        monster: opp_db_monster.clone(),
        name: monster_name,
        level:average_level};
    builder.add_opponent_monster(opp_monster);
    builder.start(end::end);
}

pub fn prepare_duel_combat(user_conn: &mut Connection, opp_conn: &mut Connection, user_monsters: &[i32], opp_monsters: &[i32]) {
    let mut db_conn = user_conn.ctx.db.get_connection().unwrap();

    let user = match get_user(user_conn, &mut db_conn) {
        Ok(user) => user,
        Err(e) => return
    };

    let opponent = match get_user(opp_conn, &mut db_conn) {
        Ok(user) => user,
        Err(e) => return
    };

    if user_monsters.is_empty() || opp_monsters.is_empty() {
        return;
    }

    // Start builder
    let user_infos = builder::UserInfos {user: user, conn: match user_conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    }};

    let opp_infos = builder::UserInfos {user: opponent, conn: match opp_conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    }};

    let mut builder : builder::CombatBuilder = builder::CombatBuilder::new(user_infos,
                                            builder::OpponentType::User(opp_infos), "duel", "city");

    //add user monsters and opponent monsters in the builder
    match add_user_monster(&mut db_conn, user_monsters, &mut builder) {
        Ok(level) => level,
        _ => return,
    };

    add_opponent_monster(&mut db_conn, opp_monsters, &mut builder);
    builder.start(end::end);
}

pub fn prepare_portal_combat(user_conn: &mut Connection, user_monster: i32, opp_monster: i32,  opp_conn: Option<Connection>) {
    let mut db_conn = user_conn.ctx.db.get_connection().unwrap();

    let user = match get_user(user_conn, &mut db_conn) {
        Ok(user) => user,
        Err(e) => return
    };

    let user_infos = builder::UserInfos {user: user, conn: match user_conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    }};

    //knowing if opponent is AI or player and construct builder arcording to it.

    let mut builder = match opp_conn {
        Some(mut opp_conn) => {
            let opponent = match get_user(&mut opp_conn, &mut db_conn) {
                Ok(user) => user,
                Err(e) => return
            };
            let opp_infos = builder::UserInfos {user: opponent, conn: match opp_conn.try_clone() {
                Ok(conn) => conn,
                Err(e) => return
            }};
            builder::CombatBuilder::new(user_infos, builder::OpponentType::User(opp_infos), "dungeon", "city")
        }
        None => builder::CombatBuilder::new(user_infos, builder::OpponentType::AI, "dungeon", "city")
    };

    //add user monsters and opponent monsters in the builder
    match add_user_monster(&mut db_conn, &[user_monster], &mut builder) {
        Ok(_) => {},
        _ => return,
    };

    add_opponent_monster(&mut db_conn, &[opp_monster], &mut builder);
    builder.start(end::end);
}
