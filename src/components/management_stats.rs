use bevy::prelude::*;
use too_big_float::BigFloat;

// Management stats - used for upgrades and persistence
#[derive(Component, Clone, Debug, PartialEq)]
pub struct BaseAttack(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct BaseDefense(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct BaseSpeed(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct BaseHp(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Experience(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Level(pub u32);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct RebirthPoints(pub BigFloat);