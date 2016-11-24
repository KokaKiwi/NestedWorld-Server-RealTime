mod db {
    pub use ::db::models::monster::Monster;
    pub use ::db::models::user::User;
    pub use ::db::models::user_monster::UserMonster;
}

#[derive(Debug, Clone)]
pub struct CombatBuilder {
    pub user: User,
    pub opponent: Opponent,
}

impl CombatBuilder {
    pub fn new(user: db::User, opponent: OpponentType) -> CombatBuilder {
        CombatBuilder {
            user: User {
                user: user,
                monsters: Vec::new(),
            },
            opponent: Opponent {
                ty: opponent,
                monsters: Vec::new(),
            },
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

    pub fn start(self) {
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user: db::User,
    pub monsters: Vec<db::UserMonster>,
}

#[derive(Debug, Clone)]
pub struct Opponent {
    pub ty: OpponentType,
    pub monsters: Vec<Monster>,
}

#[derive(Debug, Clone)]
pub enum OpponentType {
    AI,
    User(u32),
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub monster: db::Monster,
    pub name: String,
    pub level: u32,
}
