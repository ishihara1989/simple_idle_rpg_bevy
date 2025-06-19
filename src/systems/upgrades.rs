use bevy::prelude::*;
use too_big_float::BigFloat;
use crate::components::*;

// Core upgrade system - handles automatic upgrades when resources are available
pub fn upgradeable_stat_upgrade_system(
    mut player_experience_query: Query<&mut Experience, With<Player>>,
    mut upgradeable_stats: Query<(
        &UpgradeableStat,
        &mut CurrentValue,
        &BaseValue,
        &mut UpgradeLevel,
        &mut UpgradeCost,
        &UpgradeMultiplier,
        &CostMultiplier,
    )>,
) {
    let Ok(mut player_exp) = player_experience_query.single_mut() else { return };
    
    let mut upgraded = true;
    while upgraded {
        upgraded = false;
        
        for (stat, mut current_value, base_value, mut level, mut upgrade_cost, upgrade_multiplier, cost_multiplier) in upgradeable_stats.iter_mut() {
            if can_upgrade(&player_exp.0, &upgrade_cost) {
                let cost = upgrade_cost.0;
                player_exp.0 = player_exp.0 - cost;
                
                // Level up
                level.0 += 1;
                
                // Recalculate current value
                current_value.0 = recalculate_current_value(base_value, &level, upgrade_multiplier);
                
                // Update cost
                upgrade_cost.0 = upgrade_cost.0 * BigFloat::from(cost_multiplier.0);
                
                upgraded = true;
                println!("DEBUG UPGRADE: {} upgraded! New level: {}, New value: {}, Cost was: {}", 
                    stat.name, level.0, current_value.0, cost);
            }
        }
    }
}

// Reactive system - recalculates current value when base components change
pub fn update_current_value_on_change(
    mut query: Query<(
        &mut CurrentValue,
        &BaseValue,
        &UpgradeLevel,
        &UpgradeMultiplier,
    ), Or<(Changed<BaseValue>, Changed<UpgradeLevel>, Changed<UpgradeMultiplier>)>>,
) {
    for (mut current_value, base_value, level, multiplier) in query.iter_mut() {
        current_value.0 = recalculate_current_value(base_value, level, multiplier);
    }
}

// Type-safe sync systems - sync management stats to combat stats when upgrades happen
pub fn hp_sync_system(
    mut player_query: Query<(&mut MaxHp, &mut CurrentHp), With<Player>>,
    hp_stats: Query<&CurrentValue, (With<UpgradeableHp>, Changed<CurrentValue>)>,
) {
    if let Ok((mut max_hp, mut current_hp)) = player_query.single_mut() {
        for current_value in hp_stats.iter() {
            let old_max_hp = max_hp.0;
            max_hp.0 = current_value.0;
            
            // If max HP changed, update current HP to full
            if max_hp.0 != old_max_hp {
                current_hp.0 = max_hp.0;
            }
        }
    }
}

pub fn attack_sync_system(
    mut player_query: Query<&mut CombatAttack, With<Player>>,
    attack_stats: Query<&CurrentValue, (With<UpgradeableAttack>, Changed<CurrentValue>)>,
) {
    if let Ok(mut combat_attack) = player_query.single_mut() {
        for current_value in attack_stats.iter() {
            combat_attack.0 = current_value.0;
        }
    }
}

pub fn defense_sync_system(
    mut player_query: Query<&mut CombatDefense, With<Player>>,
    defense_stats: Query<&CurrentValue, (With<UpgradeableDefense>, Changed<CurrentValue>)>,
) {
    if let Ok(mut combat_defense) = player_query.single_mut() {
        for current_value in defense_stats.iter() {
            combat_defense.0 = current_value.0;
        }
    }
}

pub fn speed_sync_system(
    mut player_query: Query<&mut CombatSpeed, With<Player>>,
    speed_stats: Query<&CurrentValue, (With<UpgradeableSpeed>, Changed<CurrentValue>)>,
) {
    if let Ok(mut combat_speed) = player_query.single_mut() {
        for current_value in speed_stats.iter() {
            combat_speed.0 = current_value.0;
        }
    }
}