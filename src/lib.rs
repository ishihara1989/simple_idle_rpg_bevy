pub mod components;
pub mod systems;
pub mod events;
pub mod upgradeable_stat;
pub mod ui;
pub mod plugins;

#[cfg(test)]
pub mod tests {
    pub mod components_tests;
    pub mod systems_tests;
    pub mod integration_tests;
    pub mod real_time_combat_tests;
}

pub use components::*;
pub use systems::*;
pub use events::*;
pub use ui::*;
pub use plugins::*;
// Rename upgradeable_stat Level to avoid conflict
pub use upgradeable_stat::{
    CurrentValue, UpgradeCost, UpgradeMultiplier, CostMultiplier, BaseValue,
    Level as UpgradeLevel, UpgradeableStat, UpgradeableStatBundle,
    can_upgrade, recalculate_current_value, calculate_exponential_growth,
    upgradeable_stat_upgrade_system, update_current_value_on_change
};