use net::conn::Connection;
use super::result::CombatResult;

mod db {
    pub use ::db::models::monster::Monster;
    pub use ::db::models::user::User;
    pub use ::db::models::user_monster::UserMonster;
}

#[derive(Debug)]
pub struct CombatBuilder {
    pub user: User,
    pub opponent: Opponent,
    pub ty: String,
    pub env: String,
}

impl CombatBuilder {
    pub fn new(user: UserInfos, opponent: OpponentType, ty: &str, env: &str) -> CombatBuilder {
        CombatBuilder {
            user: User {
                infos: user,
                monsters: Vec::new(),
            },
            opponent: Opponent {
                ty: opponent,
                monsters: Vec::new(),
            },
            ty: ty.to_owned(),
            env: env.to_owned(),
        }
    }

    pub fn add_user_monster(&mut self, monster: &db::UserMonster) -> &mut Self {
        self.user.monsters.push(monster.clone());
        self
    }

    pub fn add_opponent_monster(&mut self, monster: Monster) -> &mut Self {
        self.opponent.monsters.push(monster);
        self
    }

    pub fn start<F>(self, callback: F)
        where F: Fn(CombatResult)
    {
    }
}

#[derive(Debug)]
pub struct User {
    pub infos: UserInfos,
    pub monsters: Vec<db::UserMonster>,
}

pub struct UserInfos {
    pub user: db::User,
    pub conn: Connection,
}

impl ::std::fmt::Debug for UserInfos {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("UserInfos")
         .field("user", &self.user)
         .field("conn", &self.conn.name())
         .finish()
    }
}

#[derive(Debug)]
pub struct Opponent {
    pub ty: OpponentType,
    pub monsters: Vec<Monster>,
}

#[derive(Debug)]
pub enum OpponentType {
    AI,
    User(UserInfos),
}

#[derive(Debug)]
pub struct Monster {
    pub monster: db::Monster,
    pub name: String,
    pub level: u32,
}
