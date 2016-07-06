use combat::PlayerData;
use db::models::user::User;
use net::conn::Connection;
use net::msg::Message;
use net::msg::combat::Ask;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::result::ResultData;
use net::msg::utils::rmp::ValueExt;
use net::handlers::helpers::result::send_result;

pub fn handle(conn: &mut Connection, msg: Ask) {
    let user = match conn.session() {
        Some(session) => session.user.get().unwrap().clone(),
        None => {
            send_result(conn, &msg.header, ResultData::err("NotAuthenticated", "You are not authenticated on the server", None));
            return;
        }
    };
    let db_conn = conn.ctx.db.get_connection().unwrap();
    let opponent = match User::get_by_pseudo(&db_conn, &msg.opponent) {
        Ok(Some(user)) => user,
        _ => {
            send_result(conn, &msg.header, ResultData::err("InvalidUser", "No opponent named like that.", None));
            return;
        }
    };
    let mut opponent_conn = {
        let users = conn.ctx.users.lock().unwrap_or_else(|e| e.into_inner());
        users[&(opponent.id as u32)].try_clone().unwrap()
    };

    send_result(conn, &msg.header, ResultData::ok(None));

    let mut conn = conn.try_clone().unwrap();
    ::mioco::spawn(move || {
        // Send to sender
        let avail = Available {
            header: msg.header.clone(),
            origin: Origin::Duel {
                user: ::net::msg::combat::data::User {
                    pseudo: user.pseudo.clone(),
                },
            },
        };
        let user_rx = conn.send_request(avail).unwrap();

        // Send to opponent
        let avail = Available {
            header: msg.header.clone(),
            origin: Origin::Duel {
                user: ::net::msg::combat::data::User {
                    pseudo: opponent.pseudo.clone(),
                },
            },
        };
        let opponent_rx = opponent_conn.send_request(avail).unwrap();

        let user_monsters: Vec<u32> = match user_rx.recv().unwrap() {
            Message::Result(::net::msg::ResultMessage {
                data: ResultData::Success(ref data),
                ..
            }) => {
                data.get("monsters").unwrap()
            }
            _ => { return; }
        };
        let user_monsters: Vec<_> = user_monsters.into_iter()
            .map(|user_monster_id| {
                let mut entry: ::db::models::user_monster::UserMonster = conn.ctx.db.get_model(user_monster_id as i32).unwrap().unwrap();
                entry.user.fetch(&db_conn).unwrap();
                entry.monster.fetch(&db_conn).unwrap();
                entry
            })
        .collect();

        let opponent_monsters: Vec<u32> = match opponent_rx.recv().unwrap() {
            Message::Result(::net::msg::ResultMessage {
                data: ResultData::Success(ref data),
                ..
            }) => {
                data.get("monsters").unwrap()
            }
            _ => { return; }
        };
        let opponent_monsters: Vec<_> = opponent_monsters.into_iter()
            .map(|user_monster_id| {
                let mut entry: ::db::models::user_monster::UserMonster = conn.ctx.db.get_model(user_monster_id as i32).unwrap().unwrap();
                entry.user.fetch(&db_conn).unwrap();
                entry.monster.fetch(&db_conn).unwrap();
                entry
            })
        .collect();

        let combat = {
            let mut combats = conn.ctx.combats.lock().unwrap_or_else(|e| e.into_inner());
            combats.create()
        };
        let mut combat = combat.lock().unwrap_or_else(|e| e.into_inner());
        combat.add_player(PlayerData::User {
            user: user.clone(),
            stream: conn.stream.try_clone().unwrap(),
        }, &user_monsters);
        combat.add_player(PlayerData::User {
            user: opponent.clone(),
            stream: opponent_conn.stream.try_clone().unwrap(),
        }, &opponent_monsters);
        combat.start();

        // Send start to user
        let ref user_monster = combat.monsters()[combat.players()[0].current_monster as usize];
        let ref opponent_monster = combat.monsters()[combat.players()[1].current_monster as usize];
        let start = ::net::msg::combat::Start {
            header: msg.header.clone(),
            combat_id: combat.id(),
            user: ::net::msg::combat::data::start::User {
                monster: ::net::msg::combat::data::Monster {
                    id: combat.players()[0].current_monster,
                    name: user_monster.user_monster.surname.clone(),
                    monster_id: user_monster.user_monster.monster.get().unwrap().id as u32,
                    user_monster_id: Some(user_monster.user_monster.id as u32),
                    hp: user_monster.hp as u16,
                    level: user_monster.user_monster.level as u8,
                },
            },
            opponent: ::net::msg::combat::data::start::Opponent {
                monster: ::net::msg::combat::data::Monster {
                    id: combat.players()[1].current_monster,
                    name: opponent_monster.user_monster.surname.clone(),
                    monster_id: opponent_monster.user_monster.monster.get().unwrap().id as u32,
                    user_monster_id: Some(opponent_monster.user_monster.id as u32),
                    hp: opponent_monster.hp as u16,
                    level: opponent_monster.user_monster.level as u8,
                },
                monsters_count: opponent_monsters.len() as u8,
                user: Some(::net::msg::combat::data::User {
                    pseudo: opponent.pseudo.clone(),
                }),
            },
            combat_type: "duel".to_owned(),
            env: "city".to_owned(),
            first: true,
        };
        conn.send(start).unwrap();

        // Send start to opponent
        let ref user_monster = combat.monsters()[combat.players()[0].current_monster as usize];
        let ref opponent_monster = combat.monsters()[combat.players()[1].current_monster as usize];
        let start = ::net::msg::combat::Start {
            header: msg.header.clone(),
            combat_id: combat.id(),
            user: ::net::msg::combat::data::start::User {
                monster: ::net::msg::combat::data::Monster {
                    id: combat.players()[1].current_monster,
                    name: opponent_monster.user_monster.surname.clone(),
                    monster_id: opponent_monster.user_monster.monster.get().unwrap().id as u32,
                    user_monster_id: Some(opponent_monster.user_monster.id as u32),
                    hp: opponent_monster.hp as u16,
                    level: opponent_monster.user_monster.level as u8,
                },
            },
            opponent: ::net::msg::combat::data::start::Opponent {
                monster: ::net::msg::combat::data::Monster {
                    id: combat.players()[0].current_monster,
                    name: user_monster.user_monster.surname.clone(),
                    monster_id: user_monster.user_monster.monster.get().unwrap().id as u32,
                    user_monster_id: Some(user_monster.user_monster.id as u32),
                    hp: user_monster.hp as u16,
                    level: user_monster.user_monster.level as u8,
                },
                monsters_count: user_monsters.len() as u8,
                user: Some(::net::msg::combat::data::User {
                    pseudo: user.pseudo.clone(),
                }),
            },
            combat_type: "duel".to_owned(),
            env: "city".to_owned(),
            first: false,
        };
        opponent_conn.send(start).unwrap();
    });
}
