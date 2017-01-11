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

pub fn attack(monster: &Monster, level: i32, attack: &Attack, damage: Option<u32>, opp_monster_type: ElementType) -> u32 {
    let multiplier = match attack.attack_type {
        AttackType::Attack => 1.0,
        AttackType::Defense => 0.1,
        AttackType::AttackSp => {
            let between = Range::new(0.75, 1.25);
            let mut rng = rand::thread_rng();
            between.ind_sample(&mut rng)
        }
        AttackType::DefenseSp => {
            let between = Range::new(0.05, 0.15);
            let mut rng = rand::thread_rng();
            between.ind_sample(&mut rng)
        }
    };

    let amount = match attack.attack_type {
        AttackType::Attack | AttackType::AttackSp => (level as f64 * monster.attack * multiplier * 0.25 *
                      coefficient(monster.monster_type.clone(), opp_monster_type)) as u32,
        _ => (monster.defense * level as f64 * multiplier) as u32,
    };
    debug!("attack({:?}, {:?}, level={:?}, None, {:?}) = {}", monster, level, attack, opp_monster_type, amount);
    amount
}

pub fn coefficient(attack:ElementType, defend:ElementType) -> f64 {
    //TODO : compare values
    return 1.0
}
