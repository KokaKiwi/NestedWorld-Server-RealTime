use combat::PlayerData;
use db::models::utils::Relation;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::MessageHeader;
use net::msg::Message;
use net::msg::result::ResultData;
use net::msg::utils::rmp::ValueExt;
use rand;
use rand::distributions::{IndependentSample, Range};
use super::conn::Connection;

pub fn send_random_combat(conn: &mut Connection) {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, 140);

    loop {
        let time = ::std::time::Duration::from_secs(120 + between.ind_sample(&mut rng));
        debug!("[{}] Sleeping {:?}", conn.name(), time);
        ::mioco::sleep(time);

        let mut conn = conn.try_clone().unwrap();
        ::mioco::spawn(move || start_combat(&mut conn));
    }
}

fn start_combat(conn: &mut Connection) {
    let header = MessageHeader::new();
    let db_conn = conn.ctx.db.get_connection().unwrap();

    let user = match conn.session() {
        Some(ref session) => session.user.get().unwrap().clone(),
        None => return,
    };

    let avail = Available {
        header: header.clone(),
        origin: Origin::WildMonster {
            monster_id: 1,
        },
    };
    let rx = conn.send_request(avail).unwrap();

    let monsters: Vec<u32> = match rx.recv().unwrap() {
        Message::Result(::net::msg::ResultMessage {
            data: ResultData::Success(ref data),
            ..
        }) => {
            data.get("monsters").unwrap()
        }
        _ => return,
    };
    let monsters: Vec<_> = monsters.into_iter()
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
    }, &monsters);
    let mut monster: Relation<::db::models::monster::Monster> = Relation::new(1);
    let monster_name = monster.fetch(&db_conn).unwrap().unwrap().name.clone();
    combat.add_player(PlayerData::AI, &[::db::models::user_monster::UserMonster {
        id: 0,
        surname: monster_name,
        experience: 50,
        level: 2,
        hp: 60,
        user: Relation::new(0),
        monster: monster.clone(),
    }]);
    combat.start();

    // Send start to user
    let ref user_monster = combat.monsters()[combat.players()[0].current_monster as usize];
    let ref opponent_monster = combat.monsters()[combat.players()[1].current_monster as usize];
    let start = ::net::msg::combat::Start {
        header: header.clone(),
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
            monsters_count: 1,
            user: None,
        },
        combat_type: "duel".to_owned(),
        env: "city".to_owned(),
        first: true,
    };
    conn.send(start).unwrap();
}
