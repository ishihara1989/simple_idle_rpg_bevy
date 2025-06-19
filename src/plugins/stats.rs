use bevy::prelude::*;
use crate::{
    upgradeable_stat_upgrade_system, update_current_value_on_change,
    hp_sync_system, attack_sync_system, defense_sync_system, speed_sync_system
};

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                // Upgrade and sync systems
                upgradeable_stat_upgrade_system,
                update_current_value_on_change,
                (
                    hp_sync_system,
                    attack_sync_system,
                    defense_sync_system,
                    speed_sync_system,
                ).after(upgradeable_stat_upgrade_system),
            ));
    }
}