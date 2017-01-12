use db::models::token::Session;
use db::models::user_monster::UserMonster;
use db::models::monster::Monster;
use db::models::user::User;
use db::models::utils::Model;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::conn::Connection;
use net::msg::MessageHeader;
use net::msg::Message;
use chrono::offset::utc::UTC;

mod db {
    pub use db::Connection;
}

use super::builder;
use super::end;

//add user monster to th builder and return the average level66

pub fn get_user(conn: &mut Connection, db_conn: &db::Connection) -> Result<User, String> {
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
    for &monster_id in monsters {
        let mut user_monster = match UserMonster::get_by_id(&db_conn, monster_id) {
            Ok(Some(monster)) => monster,
            Ok(None) => return Err(format!("Invalid monster id: {}", monster_id)),
            Err(e) => return Err(format!("DB error: {}", e)),
        };
        user_monster.monster.fetch(&db_conn).unwrap();
        debug!("{:?}", user_monster);
        builder.add_user_monster(&user_monster);
        average_level += user_monster.level as u32;
    }
    Ok(average_level / monsters.len() as u32)
}

pub fn add_opponent_monster(db_conn: &mut db::Connection, monsters: &[i32], builder: &mut builder::CombatBuilder) {
    for &monster in monsters {
        match UserMonster::get_by_id(db_conn, monster) {
            Ok(Some(mut user_monster)) => {
                let monster = user_monster.monster.fetch(db_conn)
                    .unwrap().expect("No monster?")
                    .clone();
                builder.add_opponent_monster (builder::Monster {
                    monster: monster.clone(),
                    user_monster: Some(user_monster.clone()),
                    name: user_monster.surname,
                    level: user_monster.level as u32
                });
            },
            _ => {debug!("opponent monster id is not correct !")},
        };
    }
}

pub fn prepare_wild_combat(conn: &mut Connection, monsters: &[i32], ai_monster: i32,
                           id: String) {
    let mut db_conn = conn.ctx.db.get_connection().unwrap();

    let user = match get_user(conn, &mut db_conn) {
        Ok(user) => user,
        Err(e) => {
            debug!("Error: {:?}", e);
            return;
        }
    };

    if monsters.is_empty() {
        debug!("No monsters?!");
        return;
    }

    // Start builder

    let user_infos = builder::UserInfos {
        user: user,
        conn: match conn.try_clone() {
            Ok(conn) => conn,
            Err(e) => {
                debug!("Connection error: {}", e);
                return;
            }
        },
    };

    let mut builder : builder::CombatBuilder = builder::CombatBuilder::new(id, user_infos,
                                                                           builder::OpponentType::AI, "wild", "city");

    //add user monsters in the builder and compute average level
    let average_level = match add_user_monster(&mut db_conn, monsters, &mut builder) {
        Ok(level) => level,
        Err(e) => {
            debug!("Unknown error: {:?}", e);
            return;
        }
    };

    //add IA monsters in the builder
    let opp_db_monster = match Monster::get_by_id(&mut db_conn, ai_monster) {
        Ok(Some(monster)) => monster,
        Ok(None) => {
            debug!("No monster for id {}", ai_monster);
            return;
        }
        Err(e) => {
            debug!("Error: {}", e);
            return;
        }
    };

    let monster_name = opp_db_monster.name.clone();
    let opp_monster = builder::Monster {
        monster: opp_db_monster.clone(),
        user_monster: None,
        name: monster_name,
        level:average_level};
    builder.add_opponent_monster(opp_monster);
    let mut builder_conn = match conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => {
            debug!("{}", e);
            return;
        }
    };
    builder.start(move |res| {
        end::end(res, &mut builder_conn)
    });
}

pub fn prepare_duel_combat(user_conn: &mut Connection, opp_conn: &mut Connection, user_monsters: &[i32], opp_monsters: &[i32],
                           uuid: String) {
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

    let mut builder : builder::CombatBuilder =
        builder::CombatBuilder::new(uuid, user_infos,
                                    builder::OpponentType::User(opp_infos), "duel", "city");

    //add user monsters and opponent monsters in the builder
    match add_user_monster(&mut db_conn, user_monsters, &mut builder) {
        Ok(level) => level,
        _ => return,
    };
    add_opponent_monster(&mut db_conn, opp_monsters, &mut builder);
    let mut builder_conn = match user_conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    };
    builder.start(move |res| {
        end::end(res, &mut builder_conn);
    });
}

pub fn prepare_portal_combat(user_conn: &mut Connection, user_monster: i32, opp_monster: i32, portal: i32, opp_conn: Option<Connection>, id: String) {
    let mut db_conn = user_conn.ctx.db.get_connection().unwrap();

    let user = match get_user(user_conn, &mut db_conn) {
        Ok(user) => user,
        Err(e) => return
    };

    let user_infos = builder::UserInfos {user: user.clone(), conn: match user_conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    }};

    //knowing if opponent is AI or player and construct builder arcording to it.

    let opponent_type = builder::OpponentType::AI;
    let mut builder = builder::CombatBuilder::new(id, user_infos, opponent_type,
                                                  "dungeon", "city");

      let msg = Available {
          header: MessageHeader::new(),
          origin: Origin::Portal {
              user: ::net::msg::combat::data::User {
                  id: user.id as u32,
                  pseudo: user.pseudo.clone(),
              },
              timeout: UTC::now()
          },
          monsters_max_count: 1,
      };

      let rx = match user_conn.send_request(msg) {
        Ok(rx) => rx,
        Err(e) => {
          debug!("Portal : {}", e);
          return;
        }
      };
      debug!("sended request for portal combat!");
      let msg = match rx.recv() {
        Ok(Message::Result(msg)) => msg,
        _ => {
          debug!("Inappropirate type of packet",);
          return;
        }
      };

    //add user monsters and opponent monsters in the builder
    match add_user_monster(&mut db_conn, &[user_monster], &mut builder) {
        Ok(_) => {},
        _ => return,
    };

    add_opponent_monster(&mut db_conn, &[opp_monster], &mut builder);
    let mut builder_conn = match user_conn.try_clone() {
        Ok(conn) => conn,
        Err(e) => return
    };

    builder.start(move |res| {
        end::end_portal(res, &mut builder_conn, portal)
    });
}
