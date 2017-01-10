use mioco::sync::mpsc as sync;
use net::conn::Connection;
use net::msg::combat;
use super::result::CombatResult;

mod db {
    pub use ::db::models::monster::Monster;
    pub use ::db::models::user::User;
    pub use ::db::models::user_monster::UserMonster;
}

pub struct Combat {
    pub id: u32,
    pub ty: String,
    pub env: String,
    pub user: User,
    pub opponent: Opponent,
    pub monsters: Monsters,
}

impl Combat {
    pub fn run(mut self) -> CombatResult {
        // Send start
        self.start();

        unimplemented!()
    }

    fn start(&mut self) {
        use net::msg::MessageHeader;
        use net::msg::combat::data::start as data;

        // Choose first monsters
        let user_monster_ref = self.user.monsters[0];
        let user_monster = user_monster_ref.get(&self.monsters);
        self.user.current = Some(user_monster_ref);

        let opponent_monster_ref = self.opponent.monsters[0];
        let opponent_monster = opponent_monster_ref.get(&self.monsters);
        self.opponent.current = Some(opponent_monster_ref);

        // Send start to user
        let user = data::User {
            monster: user_monster.as_data(user_monster_ref.id()),
        };

        let opponent = data::Opponent {
            monster: opponent_monster.as_data(opponent_monster_ref.id()),
            monsters_count: self.opponent.monsters.len() as u8,
            user: match self.opponent.ty {
                OpponentType::AI => None,
                OpponentType::User(ref infos) => Some(combat::data::user::User {
                    id: infos.user.id as u32,
                    pseudo: infos.user.pseudo.clone(),
                }),
            },
        };

        let start = combat::Start {
            header: MessageHeader::new(),
            combat_id: self.id,
            user: user,
            opponent: opponent,
            combat_type: self.ty.clone(),
            env: self.env.clone(),
            first: true,
        };
        self.user.infos.conn.send(start).unwrap();

        // Send start to opponent (if any)
        if let OpponentType::User(ref mut infos) = self.opponent.ty {
            let user = data::User {
                monster: opponent_monster.as_data(opponent_monster_ref.id()),
            };

            let opponent = data::Opponent {
                monster: user_monster.as_data(user_monster_ref.id()),
                monsters_count: self.user.monsters.len() as u8,
                user: Some(combat::data::user::User {
                    id: self.user.infos.user.id as u32,
                    pseudo: self.user.infos.user.pseudo.clone(),
                }),
            };

            let start = combat::Start {
                header: MessageHeader::new(),
                combat_id: self.id,
                user: user,
                opponent: opponent,
                combat_type: self.ty.clone(),
                env: self.env.clone(),
                first: false,
            };
            infos.conn.send(start).unwrap();
        }
    }
}

pub struct User {
    pub infos: UserInfos,
    pub monsters: Vec<MonsterRef>,
    pub current: Option<MonsterRef>,
}

pub struct Opponent {
    pub ty: OpponentType,
    pub monsters: Vec<MonsterRef>,
    pub current: Option<MonsterRef>,
}

pub enum OpponentType {
    AI,
    User(UserInfos),
}

pub struct UserInfos {
    pub user: db::User,
    pub conn: Connection,
    pub receiver: sync::Receiver<combat::Message>,
}

#[derive(Debug)]
pub struct Monster {
    pub monster: db::Monster,
    pub user_monster: Option<db::UserMonster>,
    pub name: String,
    pub level: u32,
    pub hp: u32,
}

impl Monster {
    pub fn is_ko(&self) -> bool {
        self.hp == 0
    }

    pub fn as_data(&self, id: u32) -> combat::data::Monster {
        combat::data::Monster {
            id: id,
            name: self.name.clone(),
            monster_id: self.monster.id as u32,
            user_monster_id: self.user_monster.as_ref()
                                .map(|user_monster| user_monster.id as u32),
            hp: self.hp as u16,
            level: self.level as u8,
        }
    }
}

impl From<db::UserMonster> for Monster {
    fn from(monster: db::UserMonster) -> Monster {
        Monster {
            name: monster.surname.clone(),
            level: monster.level as u32,
            hp: monster.level as u32,
            monster: monster.monster.get().expect("No monster ?!").clone(),
            user_monster: Some(monster),
        }
    }
}

impl Monster {
    #[inline]
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MonsterRef(usize);

impl MonsterRef {
    pub fn id(self) -> u32 {
        self.0 as u32
    }

    pub fn get(self, monsters: &Monsters) -> &Monster {
        unsafe { monsters.0.get_unchecked(self.0) }
    }

    pub fn get_mut(self, monsters: &mut Monsters) -> &mut Monster {
        unsafe { monsters.0.get_unchecked_mut(self.0) }
    }
}

pub struct Monsters(Vec<Monster>);

impl Monsters {
    pub fn new() -> Monsters {
        Monsters(Vec::new())
    }

    pub fn push(&mut self, monster: Monster) -> MonsterRef {
        let index = self.0.len();
        self.0.push(monster);
        MonsterRef(index)
    }

    pub fn push_all<T: Into<Monster>, I: IntoIterator<Item=T>>(&mut self,
                                                               monsters: I)
        -> Vec<MonsterRef>
    {
        monsters.into_iter()
                .map(Into::into)
                .map(|monster| self.push(monster))
                .collect()
    }
}
