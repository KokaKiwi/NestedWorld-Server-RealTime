#![allow(dead_code, unused_variables)]
use self::state::State;

pub mod store;

pub mod state {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        #[doc(hidden)]
        __InvalidState__,
        WaitingPlayers,
        Started,
        MonsterKo(u32),
        Finished,
    }

    macro_rules! action {
        ($self_:expr, $($pattern:pat => $state:expr),*) => {{
            let new_state = match $self_.0 {
                $($pattern => Some($state),)*
                _ => None,
            };

            if let Some(state) = new_state {
                $self_.0 = state;
            }

            new_state
        }};
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Machine(State);

    impl Machine {
        pub fn new() -> Machine {
            Machine(State::WaitingPlayers)
        }

        pub fn state(&self) -> State {
            self.0
        }

        pub fn start(&mut self) -> Option<State> {
            action!(self,
                State::WaitingPlayers => State::Started
            )
        }
    }

    pub fn new() -> Machine { Machine::new() }
}

pub struct Combat {
    db: ::db::Database,
    id: u32,
    state: state::Machine,
    monsters: Vec<Monster>,
    players: Vec<Player>,
}

impl Combat {
    pub fn new(db: ::db::Database, id: u32) -> Combat {
        Combat {
            db: db,
            id: id,
            state: state::new(),
            monsters: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn state(&self) -> State {
        self.state.state()
    }

    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn add_player(&mut self, player: PlayerType, monsters: &[u32]) -> ::db::error::Result<Vec<u32>> {
        Ok(Vec::new())
    }

    pub fn start(&mut self) {
    }

    pub fn attack(&mut self, player: u32, target: u32, attack: u32) {
    }

    pub fn flee(&mut self, player: u32) {
    }

    pub fn replace(&mut self, player: u32, monster: u32) {
    }

    pub fn finish(&mut self, winner: Option<u32>) {
    }
}

pub struct Monster {
    pub user_monster: ::db::models::user_monster::UserMonster,
    pub player: u32,
}

impl Monster {
    pub fn load(db: &::db::Database, id: u32, player: u32) -> ::db::error::Result<Option<Monster>> {
        let user_monster = match try!(db.get_model(id as i32)) {
            Some(user_monster) => user_monster,
            None => return Ok(None),
        };

        Ok(Some(Monster {
            user_monster: user_monster,
            player: player,
        }))
    }
}

pub enum PlayerType {
    User(u32),
    AI,
}

pub struct Player {
    pub ty: PlayerType,
    pub monsters: Vec<u32>,
    pub current_monster: u32,
    pub user: Option<::db::models::user::User>,
}

impl Player {
    pub fn new_user(db: &::db::Database, id: u32, monsters: &[u32]) -> ::db::error::Result<Option<Player>> {
        let user = match try!(db.get_model(id as i32)) {
            Some(user) => user,
            None => return Ok(None),
        };

        Ok(Some(Player {
            ty: PlayerType::User(id),
            monsters: monsters.to_owned(),
            current_monster: monsters[0],
            user: Some(user),
        }))
    }

    pub fn new_ai(monsters: &[u32]) -> Player {
        Player {
            ty: PlayerType::AI,
            monsters: monsters.to_owned(),
            current_monster: monsters[0],
            user: None,
        }
    }
}
