use db::models::user_monster::UserMonster;
use db::models::user::User;

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub user: User,
    pub monsters: Vec<UserMonster>,
    pub win: bool,
    pub average_lvl: u32,
}
