use bevy::prelude::*;
use too_big_float::BigFloat;

// Combat-related events for loose coupling between systems
#[derive(Event)]
pub struct CombatStartEvent {
    pub is_retry: bool,
}
#[derive(Event)]
pub struct CombatEndEvent {
    pub winner: CombatWinner,
    pub exp_gained: BigFloat,
}

#[derive(Event)]
pub struct PlayerDeathEvent {
    pub player_entity: Entity,
}

#[derive(Event)]
pub struct EnemyDeathEvent {
    pub enemy_entity: Entity,
    pub enemy_number: u32,
    pub exp_reward: BigFloat,
}

#[derive(Event)]
pub struct ExpGainEvent {
    pub amount: BigFloat,
}

#[derive(Event)]
pub struct NextEnemySpawnEvent {
    pub enemy_number: u32,
}

// Additional events for turn-based combat
#[derive(Event)]
pub struct TurnStartEvent {
    pub attacker: TurnAttacker,
}

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: BigFloat,
}

#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub entity_type: DeathEntityType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombatWinner {
    Player,
    Enemy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TurnAttacker {
    Player,
    Enemy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeathEntityType {
    Player,
    Enemy,
}