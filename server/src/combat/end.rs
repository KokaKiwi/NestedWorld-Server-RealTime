use super::result::CombatResult;
use super::math::experience;

pub fn end(result : CombatResult) {
    /*for monster in result.monsters {
        monster.experience += experience(result.win, result.average_lvl);
        if monster.experience > monster.experience * (monster.experience / 5) + 10 {
            monster.experience = monster.experience - monster.experience * (monster.experience / 5) + 10;
            monster.level += 1;
        }
    }*/
}
