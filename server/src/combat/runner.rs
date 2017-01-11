use combat::math;
use ctx::Context;
use db::models::utils::Model;
use mioco::sync::mpsc as sync;
use net::conn::Connection;
use net::handlers::helpers::result::send_result;
use net::msg::MessageHeader;
use net::msg::combat;
use net::msg::result::ResultData;
use super::result::CombatResult;

mod db {
    pub use ::db::models::monster::Monster;
    pub use ::db::models::user::User;
    pub use ::db::models::user_monster::UserMonster;
}

pub struct Combat {
    pub uuid: String,
    pub id: u32,
    pub ty: String,
    pub env: String,
    pub ctx: Context,
    pub user: User,
    pub opponent: Opponent,
    pub monsters: Monsters,
}

#[derive(Debug, Clone)]
enum State {
    WaitUserAttack,
    WaitOpponentAttack,
}

struct CombatLoop {
    combat: Combat,
    state: State,
}

impl Combat {
    pub fn run(mut self) -> CombatResult {
        self.start();

        CombatLoop {
            combat: self,
            state: State::WaitUserAttack,
        }.run()
    }

    fn start(&mut self) {
        use net::msg::MessageHeader;
        use net::msg::combat::data::start as data;

        // Choose first monsters
        let user_monster_ref = self.user.monsters[0];
        let user_monster = user_monster_ref.get(&self.monsters);
        debug!("User monster: {:?}", user_monster);
        self.user.current = Some(user_monster_ref);

        let opponent_monster_ref = self.opponent.monsters[0];
        let opponent_monster = opponent_monster_ref.get(&self.monsters);
        self.opponent.current = Some(opponent_monster_ref);

        let mut header = MessageHeader::new();
        header.id = Some(self.uuid.clone());

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
            header: header.clone(),
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
                header: header.clone(),
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

impl CombatLoop {
    fn run(mut self) -> CombatResult {
        loop {
            let result = match self.state {
                State::WaitUserAttack => self.wait_user_attack(),
                State::WaitOpponentAttack => self.wait_opponent_attack(),
            };
            if let Some(result) = result {
                return result;
            }
        }
    }

    fn wait_user_attack(&mut self) -> Option<CombatResult> {
        use db::models::attack::{Attack, AttackType};
        use net::msg::combat::AttackReceived;
        use net::msg::combat::attack_received as data;

        let db_conn = self.combat.ctx.db.get_connection().unwrap();

        let ref mut user = self.combat.user;
        let msg = match user.infos.receiver.recv().unwrap() {
            combat::Message::SendAttack(msg) => msg,
            msg => {
                debug!("Bad message received: {:?}", msg);
                return None;
            }
        };

        let monster_ref = user.current.expect("No monster?");
        let monster = monster_ref.get(&self.combat.monsters).clone();

        let target = match self.combat.monsters.get(msg.target) {
            Some(monster) => monster.clone(),
            None => {
                debug!("Invalid target: {}", msg.target);
                send_result(&mut user.infos.conn, &MessageHeader::new(), ResultData::err("InvalidTarget",
                                                                                         "Invalid target", None));
                return None;
            }
        };
        let attack = match Attack::get_by_id(&db_conn, msg.attack as i32).unwrap() {
            Some(attack) => attack,
            None => {
                debug!("Invalid attack: {}", msg.attack);
                send_result(&mut user.infos.conn, &MessageHeader::new(), ResultData::err("InvalidAttack",
                                                                                         "Invalid attack", None));
                return None;
            }
        };

        let damage = math::attack(&monster.monster, monster.level as i32, &attack, Some(monster.hp), target.monster.monster_type);
        match attack.attack_type {
            AttackType::Attack | AttackType::AttackSp => {
                let target = self.combat.monsters.get_mut(msg.target).unwrap();
                target.hp = target.hp.checked_sub(damage).unwrap_or(0);
            }
            _ => {
                let target = monster_ref.get_mut(&mut self.combat.monsters);
                target.hp += damage;
            }
        }

        let r_msg = AttackReceived {
            header: MessageHeader::new(),
            attack: msg.attack,
            monster: data::Monster {
                id: monster_ref.id(),
                hp: monster.hp as u16,
            },
            target: data::Monster {
                id: msg.target,
                hp: target.hp as u16,
            },
            combat: self.combat.id,
        };
        user.infos.conn.send(r_msg.clone()).unwrap();
        if let OpponentType::User(ref mut infos) = self.combat.opponent.ty {
            infos.conn.send(r_msg.clone()).unwrap();
        }

        self.state = State::WaitOpponentAttack;
        None
    }

    fn wait_opponent_attack(&mut self) -> Option<CombatResult> {
        use db::models::attack::{Attack, AttackType};
        use net::msg::combat::AttackReceived;
        use net::msg::combat::attack_received as data;

        let db_conn = self.combat.ctx.db.get_connection().unwrap();

        let monster_ref = self.combat.opponent.current.expect("No monster?");
        let monster = monster_ref.get(&self.combat.monsters).clone();

        let (target, target_id, attack) = match self.combat.opponent.ty {
            OpponentType::AI => {
                let target_ref = self.combat.user.current.expect("No monster?");
                let target = target_ref.get(&mut self.combat.monsters).clone();

                let attack = Attack::get_by_id(&db_conn, 1).unwrap().expect("meh");

                (target, target_ref.id(), attack)
            }
            OpponentType::User(ref mut infos) => {
                let msg = match infos.receiver.recv().unwrap() {
                    combat::Message::SendAttack(msg) => msg,
                    msg => {
                        debug!("Bad message received: {:?}", msg);
                        return None;
                    }
                };

                let target = match self.combat.monsters.get(msg.target) {
                    Some(monster) => monster.clone(),
                    None => {
                        debug!("Invalid target: {}", msg.target);
                        send_result(&mut infos.conn, &MessageHeader::new(), ResultData::err("InvalidTarget",
                                                                                                 "Invalid target", None));
                        return None;
                    }
                };
                let attack = match Attack::get_by_id(&db_conn, msg.attack as i32).unwrap() {
                    Some(attack) => attack,
                    None => {
                        debug!("Invalid attack: {}", msg.attack);
                        send_result(&mut infos.conn, &MessageHeader::new(), ResultData::err("InvalidAttack",
                                                                                                 "Invalid attack", None));
                        return None;
                    }
                };

                (target, msg.target, attack)
            }
        };

        let damage = math::attack(&monster.monster, monster.level as i32, &attack, Some(monster.hp), target.monster.monster_type);
        match attack.attack_type {
            AttackType::Attack | AttackType::AttackSp => {
                let target = self.combat.monsters.get_mut(target_id).unwrap();
                target.hp = target.hp.checked_sub(damage).unwrap_or(0);
            }
            _ => {
                let target = monster_ref.get_mut(&mut self.combat.monsters);
                target.hp += damage;
            }
        }

        let r_msg = AttackReceived {
            header: MessageHeader::new(),
            attack: attack.id as u32,
            monster: data::Monster {
                id: monster_ref.id(),
                hp: monster.hp as u16,
            },
            target: data::Monster {
                id: target_id,
                hp: target.hp as u16,
            },
            combat: self.combat.id,
        };
        self.combat.user.infos.conn.send(r_msg.clone()).unwrap();
        if let OpponentType::User(ref mut infos) = self.combat.opponent.ty {
            infos.conn.send(r_msg.clone()).unwrap();
        }

        self.state = State::WaitUserAttack;
        None
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

#[derive(Debug, Clone)]
pub struct Monster {
    pub monster: db::Monster,
    pub user_monster: Option<db::UserMonster>,
    pub name: String,
    pub level: u32,
    pub max_hp: u32,
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
        let m_monster = monster.monster.get().expect("No monster ?!").clone();
        Monster {
            name: monster.surname.clone(),
            level: monster.level as u32,
            hp: m_monster.hp as u32,
            max_hp: m_monster.hp as u32,
            monster: m_monster,
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

    pub fn get(&self, id: u32) -> Option<&Monster> {
        self.0.get(id as usize)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Monster> {
        self.0.get_mut(id as usize)
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
