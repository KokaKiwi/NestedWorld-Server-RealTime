pub use self::ask::Ask;
pub use self::attack_received::AttackReceived;
pub use self::available::Available;
pub use self::end::End;
pub use self::flee::Flee;
pub use self::monster_ko::MonsterKo;
pub use self::monster_replaced::MonsterReplaced;
pub use self::send_attack::SendAttack;
pub use self::start::Start;

pub mod data;

pub mod ask;
pub mod attack_received;
pub mod available;
pub mod end;
pub mod flee;
pub mod monster_ko;
pub mod monster_replaced;
pub mod send_attack;
pub mod start;

message!(Message {
    type "combat:ask" => Ask(Ask),
    type "combat:available" => Available(Available),
    type "combat:start" => Start(Start),
    type "combat:send-attack" => SendAttack(SendAttack),
    type "combat:attack-received" => AttackReceived(AttackReceived),
    type "combat:flee" => Flee(Flee),
    type "combat:end" => End(End),
    type "combat:monster-replaced" => MonsterReplaced(MonsterReplaced),
    ref MonsterKo(self::monster_ko::Message),
});
