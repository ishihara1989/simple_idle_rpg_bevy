use bevy::prelude::*;
use too_big_float::BigFloat;

// Combat stats - temporary during battle, derived from management stats
#[derive(Component, Clone, Debug, PartialEq)]
pub struct CurrentHp(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct MaxHp(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct CombatAttack(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct CombatDefense(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct CombatSpeed(pub BigFloat);

// Combat state tracking
#[derive(Component)]
pub struct CombatTimer {
    pub timer: Timer,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct ExpReward(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct EnemyNumber(pub u32);