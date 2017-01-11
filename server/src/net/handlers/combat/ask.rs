use net::conn::Connection;
use net::msg::combat::Ask;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::Message;
use mioco::sync::mpsc::Receiver;
use net::msg::result::ResultData;
use net::msg::utils::rmp::ValueExt;
use net::handlers::helpers::result::send_result;
use db::models::user::User;
use std::result;
use combat::prepare::prepare_duel_combat;

pub fn find_connected_user(conn: &Connection, pseudo: &str) -> result::Result<Connection, ResultData> {
    let db_conn = conn.ctx.db.get_connection().unwrap();
    let opponent = match User::get_by_pseudo(&db_conn, pseudo) {
        Ok(Some(user)) => user,
        _ => return Err(ResultData::err("InvalidUser", "No opponent named like that.", None)),
    };
    drop(db_conn);
    let opponent_conn = match {
        let users = conn.ctx.users.lock().unwrap_or_else(|e| e.into_inner());
        users.get(&(opponent.id as u32)).map(|conn| conn.try_clone().unwrap())
    } {
        Some(conn) => conn,
        None => return Err(ResultData::err("InvalidUser", "User not connected", None)),
    };
    return Ok(opponent_conn);
}

pub fn get_combat_monsters(user_rx: Receiver<Message>, _conn: &Connection) -> result::Result<Vec<i32>, bool> {
    let user_monsters: Vec<i32> = match user_rx.recv().unwrap() {
        Message::Result(::net::msg::ResultMessage {
            data: ResultData::Success(ref data),
            ..
        }) => {
            data.get("monsters").unwrap_or(vec![])
        }
        _ => { return Err(true) }
    };
    return Ok(user_monsters);
}

pub fn handle(conn: &mut Connection, msg: Ask) {

    // look if user is connected
    let user = match conn.session() {
        Some(session) => session.user.get().unwrap().clone(),
        None => {
            send_result(conn, &msg.header, ResultData::err("NotAuthenticated", "You are not authenticated on the server", None));
            return;
        }
    };

    // look if opponnent exist
    let mut opponent_conn = match find_connected_user(conn, &msg.opponent) {
        Ok(opponent_conn) => opponent_conn,
        Err(e) => {
            send_result(conn, &msg.header, e);
            return
        }
    };
    let opponent_user = opponent_conn.session().expect("No user session?!")
                            .user.get().unwrap().clone();

    send_result(conn, &msg.header, ResultData::ok(None));

    // send a combat available to the two players

    let mut conn = conn.try_clone().unwrap();
    let mut msg = msg;
    let uuid = msg.header.ensure_id();

    ::mioco::spawn(move || {
        // Send to sender
        let avail_opp = Available {
            header: msg.header.clone(),
            origin: Origin::Duel {
                user: ::net::msg::combat::data::User {
                    id: user.id as u32,
                    pseudo: user.pseudo.clone(),
                },
            },
            monsters_max_count: 4,
        };

        let avail_user = Available {
            header: msg.header.clone(),
            origin: Origin::Duel {
                user: ::net::msg::combat::data::User {
                    id: opponent_user.id as u32,
                    pseudo: msg.opponent,
                },
            },
            monsters_max_count: 4,
        };

        let user_rx = handler_try!(conn.send_request(avail_user));

        // Send to opponent
        let opponent_rx = handler_try!(opponent_conn.send_request(avail_opp));

        //compute user monster
        let user_monsters = match get_combat_monsters(user_rx, &conn) {
            Ok(um) => um,
            Err(_) => return
        };

        let opponent_monsters = match get_combat_monsters(opponent_rx, &conn) {
            Ok(om) => om,
            Err(_) => return
        };

        prepare_duel_combat(&mut conn, &mut opponent_conn, &user_monsters, &opponent_monsters, uuid);
    });
}
