use db::models::user_monster::UserMonster;
use db::models::user::User;
use db::models::attack::Attack;
use db::models::attack::AttackType;
use db::models::monster::Monster;
use rand;
use rand::distributions::{IndependentSample, Range};

fn experience(win: bool, monster: &UserMonster, opponent: &User) -> i32 {
    let multiplier = if win { 0.5 } else { 1.0 };
    return ((monster.level * 2) as f32 * multiplier) as i32
}

fn attack(umonster: &UserMonster, monster: &Monster, attack: &Attack, damage: Option<u32>) -> u32 {
    let multiplier = match attack.attack_type {
     AttackType::Attack => 1.0,
     AttackType::Defense => 0.1,
     AttackType::AttackSp => {
        let between = Range::new(0.75, 1.25);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng) },
    AttackType::DefenseSp => {
        let between = Range::new(0.05, 0.15);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng) }
    };

    if attack.attack_type == AttackType::Attack || attack.attack_type == AttackType::AttackSp {
        return umonster.level as u32 * monster.attack as u32 / monster.defense as u32 * multiplier  as u32
    }
    else {
        match damage {
            Some(dam) => return dam.checked_sub(monster.defense  as u32 * umonster.level as u32 * multiplier as u32).unwrap_or(0),
            None => return 0
        }
    }
}
