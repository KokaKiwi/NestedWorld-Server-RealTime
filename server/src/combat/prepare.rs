use net::conn::Connection;
use db::models::token::Session;
use db::models::user_monster::UserMonster;
use db::models::user::User;
use db::models::wild_monster::WildMonster;
use db::models::utils::Model;

enum CombatType {
    Portal,
    Wild,
    Duel
}

enum PlayerData {
    User {
        user: User,
        monsters: Vec<UserMonster>
    },
    AIPortal {
        monster: UserMonster
    },
    AIWild {
        monster: WildMonster
    },
}

struct CombatInfos {
    player: PlayerData,
    opponent: PlayerData,
    combat_type: CombatType,
}

pub fn prepare_wild_combat(conn: &mut Connection, monsters: Vec<i32>) {
    // Get User
    let session : Session = match conn.session() {
        Some(session) => session,
        None => return,
    };
    let id : i32 = match session.user.get() {
        Some(user) => user.id,
        _ => return,
    };
    let db_conn = conn.ctx.db.get_connection().unwrap();
    let user = match User::get_by_id(&db_conn, id) {
        Ok(Some(user)) => user,
        _ => return,
    };
    let mut user_monsters = Vec::new();
    let mut levels = Vec::new();

    if monsters.is_empty() {
        return;
    }

    //Get Monsters of the user
    for monster in monsters {
        let user_monster = match UserMonster::get_by_id(&db_conn, monster) {
            Ok(Some(monster)) => monster,
            _ => return,
        };
        levels.push(user_monster.level as u32);
        user_monsters.push(user_monster);
    }

    // Get the average of the level
    let mut total : u32 = 0;
    for level in &levels {
        total += *level;
    }

    let average_level: u32 = total / (levels.len()) as u32;

    //Get Monster of the AI
    let wild_monster : WildMonster =  match WildMonster::generate(&db_conn, average_level) {
        Ok(Some(monster)) => monster,
        _ => return,
    };

    let infos : CombatInfos = CombatInfos {
        combat_type: CombatType::Wild,
        player:  PlayerData::User {
            user: user,
            monsters: user_monsters,
        },
        opponent: PlayerData::AIWild {
            monster: wild_monster,
        }
    };

    //send infos to the combat :D
}

pub fn prepare_portal_combat() {

}

pub fn prepare_duel_combat() {

}
