use mioco::sync::mpsc as sync;
use net::conn::Connection;
use net::msg::combat::Message;
use super::result::CombatResult;
use super::runner as run;

mod db {
    pub use ::db::models::monster::Monster;
    pub use ::db::models::user::User;
    pub use ::db::models::user_monster::UserMonster;
}

#[derive(Debug)]
pub struct CombatBuilder {
    pub uuid: String,
    pub user: User,
    pub opponent: Opponent,
    pub ty: String,
    pub env: String,
}

impl CombatBuilder {
    pub fn new(uuid: String, user: UserInfos, opponent: OpponentType, ty: &str, env: &str) -> CombatBuilder {
        CombatBuilder {
            uuid: uuid,
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
        where F: FnOnce(CombatResult) + Send + 'static
    {
        let mut ctx = self.user.infos.conn.ctx.clone();
        let mut monsters = run::Monsters::new();

        let (user, user_handle) = self.user.build(&mut monsters);
        let (opponent, opponent_handle) = self.opponent.build(&mut monsters);

        let handle = CombatHandle {
            user: user_handle,
            opponent: opponent_handle,
        };
        let id = ctx.add_combat(handle);

        let combat = run::Combat {
            uuid: self.uuid,
            id: id,
            ty: self.ty,
            env: self.env,
            ctx: ctx,
            user: user,
            opponent: opponent,
            monsters: monsters,
        };
        ::mioco::spawn(move || {
            callback(combat.run())
        });
    }
}

#[derive(Clone)]
pub struct CombatHandle {
    pub user: UserHandle,
    pub opponent: Option<UserHandle>,
}

impl CombatHandle {
    pub fn send(&mut self, id: u32, msg: &Message) {
        let msg = msg.clone();

        if id == self.user.id {
            self.user.sender.send(msg).unwrap()
        } else if let Some(ref mut opponent) = self.opponent {
            if id == opponent.id {
                opponent.sender.send(msg).unwrap()
            }
        }
    }
}

#[derive(Clone)]
pub struct UserHandle {
    pub id: u32,
    pub sender: sync::Sender<Message>,
}

#[derive(Debug)]
pub struct User {
    pub infos: UserInfos,
    pub monsters: Vec<db::UserMonster>,
}

impl User {
    pub fn build(self, monsters: &mut run::Monsters) -> (run::User, UserHandle) {
        let (infos, handle) = self.infos.build();

        let user = run::User {
            infos: infos,
            monsters: monsters.push_all(self.monsters),
            current: None,
        };

        (user, handle)
    }
}

#[derive(Debug)]
pub struct Opponent {
    pub ty: OpponentType,
    pub monsters: Vec<Monster>,
}

impl Opponent {
    fn build(self, monsters: &mut run::Monsters) -> (run::Opponent, Option<UserHandle>) {
        let (ty, handle) = self.ty.build();

        let opponent = run::Opponent {
            ty: ty,
            monsters: monsters.push_all(self.monsters),
            current: None,
        };

        (opponent, handle)
    }
}

#[derive(Debug)]
pub enum OpponentType {
    AI,
    User(UserInfos),
}

impl OpponentType {
    fn build(self) -> (run::OpponentType, Option<UserHandle>) {
        match self {
            OpponentType::AI => (run::OpponentType::AI, None),
            OpponentType::User(infos) => {
                let (infos, handle) = infos.build();
                let ty = run::OpponentType::User(infos);
                (ty, Some(handle))
            }
        }
    }
}

#[derive(Debug)]
pub struct UserInfos {
    pub user: db::User,
    pub conn: Connection,
}

impl UserInfos {
    fn build(self) -> (run::UserInfos, UserHandle) {
        let (tx, rx) = sync::channel();

        let handle = UserHandle {
            id: self.user.id as u32,
            sender: tx,
        };
        let infos = run::UserInfos {
            user: self.user,
            conn: self.conn,
            receiver: rx,
        };

        (infos, handle)
    }
}

#[derive(Debug)]
pub struct Monster {
    pub monster: db::Monster,
    pub name: String,
    pub level: u32,
}

impl Into<run::Monster> for Monster {
    fn into(self) -> run::Monster {
        run::Monster {
            user_monster: None,
            name: self.name,
            level: self.level,
            hp: self.monster.hp as u32,
            max_hp: self.monster.hp as u32,
            monster: self.monster,
        }
    }
}
