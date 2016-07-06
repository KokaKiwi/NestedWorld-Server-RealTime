use combat::PlayerData;
use combat::state::State;
use net::conn::Connection;
use net::msg::MessagePart;
use net::msg::combat::SendAttack;
use net::msg::result::ResultData;
use net::msg::combat::monster_ko::MonsterKo;
use net::handlers::helpers::result::send_result;
use rmp::encode::value::write_value;

pub fn handle(conn: &mut Connection, msg: SendAttack) {
    let session = match conn.session() {
        Some(session) => session.clone(),
        None => {
            send_result(conn, &msg.header, ResultData::err("NotAuthenticated", "You are not authenticated on the server", None));
            return;
        }
    };
    let user = session.user.get().unwrap(); // If the session is here, then the user is here too.
    let combat = {
        let combats = conn.ctx.combats.lock().unwrap_or_else(|e| e.into_inner());
        combats.get(msg.combat).map(|combat| combat.clone())
    };
    let combat = match combat {
        Some(combat) => combat,
        None => {
            send_result(conn, &msg.header, ResultData::err("InvalidCombat", "Invalid combat ID", None));
            return;
        }
    };
    let mut combat = combat.lock().unwrap_or_else(|e| e.into_inner());

    let player_id = match combat.players().iter().enumerate().find(|&(_, ref player)| {
        match player.data {
            PlayerData::User {
                user: ref o_user,
                ..
            } => o_user.id == user.id,
            _ => false,
        }
    }) {
        Some((player_id, _)) => player_id as u32,
        None => {
            send_result(conn, &msg.header, ResultData::err("InvalidCombat", "Invalid combat ID", None));
            return;
        }
    };

    combat.attack(player_id, msg.target, msg.attack);

    if !send_result(conn, &msg.header, ResultData::ok(None)) {
        return;
    }

    {
        let ref player = combat.players()[player_id as usize];
        let ref attacker = combat.monsters()[player.current_monster as usize];
        let ref attackee = combat.monsters()[msg.target as usize];
        let ar_msg = ::net::msg::combat::AttackReceived {
            header: msg.header.clone(),
            combat: msg.combat,
            attack: msg.attack,
            monster: ::net::msg::combat::data::attack_received::Monster {
                id: player.current_monster,
                hp: attacker.hp as u16,
            },
            target: ::net::msg::combat::data::attack_received::Monster {
                id: msg.target,
                hp: attackee.hp as u16,
            },
        };
        let ar_value = ar_msg.value();
        for player in combat.players() {
            match player.data {
                PlayerData::User {
                    ref stream,
                    ..
                } => {
                    if let Ok(mut stream) = stream.try_clone() {
                        let _ = write_value(&mut stream, &ar_value);
                    }
                }
                _ => {}
            }
        }
    }

    // Handle KO
    if let State::MonsterKo(monster_id) = combat.state() {
        let ko_msg = MonsterKo {
            header: msg.header.clone(),
            combat: msg.combat,
            monster: monster_id,
        };
        let ko_value = ko_msg.value();

        let owner_id = combat.monsters()[msg.target as usize].player;

        for player in combat.players().iter().enumerate()
                            .filter_map(|(id, player)| if id as u32 != owner_id { Some(player) } else { None })
        {
            match player.data {
                PlayerData::User {
                    ref stream,
                    ..
                } => {
                    if let Ok(mut stream) = stream.try_clone() {
                        let _ = write_value(&mut stream, &ko_value);
                    }
                }
                _ => {}
            }
        }

        let mut conn = handler_try!(conn.try_clone());
        ::mioco::spawn(move || {
            let rx = conn.send_request(ko_msg).unwrap();
        });
    }
}
