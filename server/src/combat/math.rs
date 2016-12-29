use db::models::user_monster::UserMonster;
use db::models::attack::Attack;
use db::models::attack::AttackType;
use db::models::monster::Monster;
use db::models::commons::ElementType;
use rand;
use rand::distributions::{IndependentSample, Range};

pub fn experience(win: bool, average_lvl: u32) -> i32 {
    let multiplier = if win { 0.5 } else { 1.0 };
    return ((average_lvl * 2) as f32 * multiplier) as i32
}

pub fn attack(umonster: &UserMonster, monster: &Monster, attack: &Attack, damage: Option<u32>, opp_monster_type: ElementType) -> u32 {
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
        return (umonster.level as f64 * monster.attack / monster.defense * multiplier *
                coefficient(monster.monster_type.clone(), opp_monster_type))as u32
    }
    else {
        match damage {
            Some(dam) => return dam.checked_sub((monster.defense * umonster.level as f64 * multiplier) as u32).unwrap_or(0),
            None => return 0
        }
    }
}

pub fn coefficient(attack:ElementType, defend:ElementType) -> f64 {
    //TODO : compare values
    return 1.0
}
