use bevy::prelude::*;
use too_big_float::BigFloat;

// Individual components - loosely coupled and reusable
#[derive(Component, Clone, Debug, PartialEq)]
pub struct CurrentValue(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct UpgradeCost(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct UpgradeMultiplier(pub f64);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct CostMultiplier(pub f64);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct BaseValue(pub BigFloat);

// Renamed to avoid conflict with management_stats::Level
#[derive(Component, Clone, Debug, PartialEq)]
pub struct UpgradeLevel(pub u32);

// Marker component - groups upgradeable stats (kept for backward compatibility)
#[derive(Component, Clone, Debug)]
pub struct UpgradeableStat {
    pub name: String,
}

impl UpgradeableStat {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}

// Type-safe marker components for each stat type
#[derive(Component, Clone, Debug)]
pub struct UpgradeableHp;

#[derive(Component, Clone, Debug)]
pub struct UpgradeableAttack;

#[derive(Component, Clone, Debug)]
pub struct UpgradeableDefense;

#[derive(Component, Clone, Debug)]
pub struct UpgradeableSpeed;

// Entity creation helper bundles
#[derive(Bundle)]
pub struct UpgradeableStatBundle {
    pub upgradeable_stat: UpgradeableStat,
    pub current_value: CurrentValue,
    pub base_value: BaseValue,
    pub level: UpgradeLevel,
    pub upgrade_cost: UpgradeCost,
    pub upgrade_multiplier: UpgradeMultiplier,
    pub cost_multiplier: CostMultiplier,
}

// Type-safe bundles for each stat type
#[derive(Bundle)]
pub struct UpgradeableHpBundle {
    pub marker: UpgradeableHp,
    pub current_value: CurrentValue,
    pub base_value: BaseValue,
    pub level: UpgradeLevel,
    pub upgrade_cost: UpgradeCost,
    pub upgrade_multiplier: UpgradeMultiplier,
    pub cost_multiplier: CostMultiplier,
}

#[derive(Bundle)]
pub struct UpgradeableAttackBundle {
    pub marker: UpgradeableAttack,
    pub current_value: CurrentValue,
    pub base_value: BaseValue,
    pub level: UpgradeLevel,
    pub upgrade_cost: UpgradeCost,
    pub upgrade_multiplier: UpgradeMultiplier,
    pub cost_multiplier: CostMultiplier,
}

#[derive(Bundle)]
pub struct UpgradeableDefenseBundle {
    pub marker: UpgradeableDefense,
    pub current_value: CurrentValue,
    pub base_value: BaseValue,
    pub level: UpgradeLevel,
    pub upgrade_cost: UpgradeCost,
    pub upgrade_multiplier: UpgradeMultiplier,
    pub cost_multiplier: CostMultiplier,
}

#[derive(Bundle)]
pub struct UpgradeableSpeedBundle {
    pub marker: UpgradeableSpeed,
    pub current_value: CurrentValue,
    pub base_value: BaseValue,
    pub level: UpgradeLevel,
    pub upgrade_cost: UpgradeCost,
    pub upgrade_multiplier: UpgradeMultiplier,
    pub cost_multiplier: CostMultiplier,
}

// Bundle constructors
impl UpgradeableStatBundle {
    pub fn new(
        name: impl Into<String>,
        base_value: BigFloat,
        initial_cost: BigFloat,
        upgrade_multiplier: f64,
        cost_multiplier: f64,
    ) -> Self {
        Self {
            upgradeable_stat: UpgradeableStat::new(name),
            current_value: CurrentValue(base_value),
            base_value: BaseValue(base_value),
            level: UpgradeLevel(0),
            upgrade_cost: UpgradeCost(initial_cost),
            upgrade_multiplier: UpgradeMultiplier(upgrade_multiplier),
            cost_multiplier: CostMultiplier(cost_multiplier),
        }
    }
}

impl UpgradeableHpBundle {
    pub fn new(
        base_value: BigFloat,
        initial_cost: BigFloat,
        upgrade_multiplier: f64,
        cost_multiplier: f64,
    ) -> Self {
        Self {
            marker: UpgradeableHp,
            current_value: CurrentValue(base_value),
            base_value: BaseValue(base_value),
            level: UpgradeLevel(0),
            upgrade_cost: UpgradeCost(initial_cost),
            upgrade_multiplier: UpgradeMultiplier(upgrade_multiplier),
            cost_multiplier: CostMultiplier(cost_multiplier),
        }
    }
}

impl UpgradeableAttackBundle {
    pub fn new(
        base_value: BigFloat,
        initial_cost: BigFloat,
        upgrade_multiplier: f64,
        cost_multiplier: f64,
    ) -> Self {
        Self {
            marker: UpgradeableAttack,
            current_value: CurrentValue(base_value),
            base_value: BaseValue(base_value),
            level: UpgradeLevel(0),
            upgrade_cost: UpgradeCost(initial_cost),
            upgrade_multiplier: UpgradeMultiplier(upgrade_multiplier),
            cost_multiplier: CostMultiplier(cost_multiplier),
        }
    }
}

impl UpgradeableDefenseBundle {
    pub fn new(
        base_value: BigFloat,
        initial_cost: BigFloat,
        upgrade_multiplier: f64,
        cost_multiplier: f64,
    ) -> Self {
        Self {
            marker: UpgradeableDefense,
            current_value: CurrentValue(base_value),
            base_value: BaseValue(base_value),
            level: UpgradeLevel(0),
            upgrade_cost: UpgradeCost(initial_cost),
            upgrade_multiplier: UpgradeMultiplier(upgrade_multiplier),
            cost_multiplier: CostMultiplier(cost_multiplier),
        }
    }
}

impl UpgradeableSpeedBundle {
    pub fn new(
        base_value: BigFloat,
        initial_cost: BigFloat,
        upgrade_multiplier: f64,
        cost_multiplier: f64,
    ) -> Self {
        Self {
            marker: UpgradeableSpeed,
            current_value: CurrentValue(base_value),
            base_value: BaseValue(base_value),
            level: UpgradeLevel(0),
            upgrade_cost: UpgradeCost(initial_cost),
            upgrade_multiplier: UpgradeMultiplier(upgrade_multiplier),
            cost_multiplier: CostMultiplier(cost_multiplier),
        }
    }
}

// Utility functions
pub fn can_upgrade(
    available_resource: &BigFloat,
    upgrade_cost: &UpgradeCost,
) -> bool {
    available_resource >= &upgrade_cost.0
}

pub fn recalculate_current_value(
    base_value: &BaseValue,
    level: &UpgradeLevel,
    multiplier: &UpgradeMultiplier,
) -> BigFloat {
    let mut result = base_value.0;
    for _ in 0..level.0 {
        result = result * BigFloat::from(multiplier.0);
    }
    result
}

// Backward compatibility function
pub fn calculate_exponential_growth(
    base: BigFloat,
    multiplier: f64,
    level: u32,
) -> BigFloat {
    let multiplier_bf = BigFloat::from(multiplier);
    let mut result = base;
    for _ in 0..level {
        result = result * multiplier_bf;
    }
    result
}