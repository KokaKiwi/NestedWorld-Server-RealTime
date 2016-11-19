use db::models::user_monster::UserMonster;
use db::models::user::User;
use db::models::attack::Attack;
use rand::Rng;

fn experience(win: bool, monster: &UserMonster, opponent: &User) -> u32 {
    let multiplier = if win { 0.5 } else { 1 };
    return (monster.level * 2) * multiplier
}

fn attack(monster: &UserMonster, attack: &Attack, damage: Option<u32>) -> u32 {
    let multiplier = match attack.type {
     AttackType.Attack => 1.0
     AttackType.Defense => 0.1
     AttackType.AttackSp => {
        let between = Range::new(0.75, 1.25);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng) }
    AttackType.DefenseSp => {
        let between = Range::new(0.05, 0.15);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng) }

    if attack.type == AttackType.Attack || attack.type == AttackType.AttackSp {
        return monster.level * monster.monster.attack / monster.monster.defense * multiplier
    }
    else {
        match damage {
            Some(dam) => return dam.checked_sub(monster.monster.defense * monster.level * multiplier).unwrap_or(0)
            None => return 0
        }
    }
}

fn
