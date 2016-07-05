use combat::state::State;
use net::conn::Connection;
use net::msg::combat::SendAttack;
use net::msg::result::ResultData;
use net::msg::combat::monster_ko::MonsterKo;
use net::handlers::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: SendAttack) {
    handle_with_result(conn, &msg.header, |conn| {
        match conn.session {
            Some(_) => {
                let o_combat;
                match conn.ctx.combats.lock() {
                    Ok(combats) => { o_combat = combats.get(msg.combat); }
                    Err(e) => {
                        debug!("Alert ! Cannot lock combats ! {}", e);
                        return ResultData::err("ServerError", "A value has not been locked", None);
                    }
                };
                let id;
                match conn.session {
                    Some(ref session) => {
                        match session.user.get() {
                            Some(user) => {
                                id = user.id;
                            }
                            None => {
                                debug!("Cannot get current user");
                                return ResultData::err("ServerError", "Cannot get current user", None);
                            }
                        }
                    }
                    None => {
                        debug!("Session is empty");
                        return ResultData::err("InvalidCombat", "Session doesn't exist", None);
                    }
                }

                match o_combat {
                    Some(m_combat) => {
                        match m_combat.lock() {
                            Ok(mut combat) => {
                                combat.attack(id as u32, msg.target, msg.attack);
                            }
                            Err(e) => {
                                debug!("Alert ! Cannot lock combat ! {}", e);
                                return ResultData::err("ServerError", "A value has not been locked", None);
                            }
                        }
                    }
                    None => {
                        debug!("Combat is empty");
                        return ResultData::err("InvalidCombat", "Combat doesn't exist", None);
                    }
                }
                return ResultData::ok(None)
            },
            None => { return ResultData::err("NotAuthenticated", "You are not authenticated on the server", None)}
        }
    });

    let o_combat;
    match conn.ctx.combats.lock() {
        Ok(combats) => { o_combat = combats.get(msg.combat); }
        Err(e) => {
            debug!("Alert ! Cannot lock combats ! {}", e);
            return;
        }
    };
    match o_combat {
        Some(m_combat) => {
            match m_combat.lock() {
                Ok(combat) => {
                    if combat.state() == State::MonsterKo(msg.target) {
                        let ko = MonsterKo {
                            header: msg.header,
                            monster: msg.target,
                        };

                        match conn.send(ko) {
                            Ok(_) => {}
                            Err(e) => { debug!("Error when sending monsterko: {}", e); }
                        }
                    }
                }
                Err(e) => {
                    debug!("Alert ! Cannot lock combat ! {}", e);
                    return;
                }
            }
        }
        None => { debug!("Combat is empty") }
    }
}
