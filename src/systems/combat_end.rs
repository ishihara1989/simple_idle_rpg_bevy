use bevy::prelude::*;
use too_big_float::BigFloat;
use crate::components::*;
use crate::events::*;
use crate::systems::initialization::{rebirth_player_system, spawn_enemy};

// Detect deaths and handle the aftermath
pub fn death_detection_system(
    mut death_events: EventReader<DeathEvent>,
    mut player_death_events: EventWriter<PlayerDeathEvent>,
    mut enemy_death_events: EventWriter<EnemyDeathEvent>,
    enemy_query: Query<(&EnemyNumber, &ExpReward), With<Enemy>>,
) {
    for death in death_events.read() {
        match death.entity_type {
            DeathEntityType::Player => {
                player_death_events.send(PlayerDeathEvent {
                    player_entity: death.entity,
                });
            }
            DeathEntityType::Enemy => {
                if let Ok((enemy_number, exp_reward)) = enemy_query.get(death.entity) {
                    enemy_death_events.send(EnemyDeathEvent {
                        enemy_entity: death.entity,
                        enemy_number: enemy_number.0,
                        exp_reward: exp_reward.0.clone(),
                    });
                }
            }
        }
    }
}

// Handle enemy deaths - award EXP and spawn next enemy
pub fn enemy_death_system(
    mut commands: Commands,
    mut enemy_death_events: EventReader<EnemyDeathEvent>,
    mut exp_events: EventWriter<ExpGainEvent>,
    mut next_enemy_events: EventWriter<NextEnemySpawnEvent>,
) {
    for death in enemy_death_events.read() {
        // Remove dead enemy
        commands.entity(death.enemy_entity).despawn();
        
        // Award experience
        exp_events.send(ExpGainEvent {
            amount: death.exp_reward.clone(),
        });
        
        // Spawn next enemy
        next_enemy_events.send(NextEnemySpawnEvent {
            enemy_number: death.enemy_number + 1,
        });
        
        println!("Enemy {} defeated! Gained {} EXP", death.enemy_number, death.exp_reward);
    }
}

// Handle player deaths - trigger rebirth
pub fn player_death_system(
    mut commands: Commands,
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut game_state: ResMut<GameState>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    // Process only the first death event to avoid moving commands multiple times
    if let Some(death) = player_death_events.read().next() {
        println!("Game Over! Starting rebirth...");
        
        let rebirth_gain = BigFloat::from(game_state.current_enemy_number as f64);
        println!("Gained {} rebirth points", rebirth_gain);
        
        // Remove player and enemies
        commands.entity(death.player_entity).despawn();
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn();
        }
        
        // Reset game state
        game_state.current_enemy_number = 1;
        game_state.is_game_over = false;
        
        // Rebirth player with enhanced stats
        rebirth_player_system(&mut commands, rebirth_gain);
        
        // Spawn first enemy
        spawn_enemy(&mut commands, 1);
    }
}

// Handle experience gain
pub fn exp_gain_system(
    mut exp_events: EventReader<ExpGainEvent>,
    mut player_query: Query<&mut Experience, With<Player>>,
) {
    for exp in exp_events.read() {
        if let Ok(mut player_exp) = player_query.get_single_mut() {
            player_exp.0 += exp.amount.clone();
            println!("Gained {} EXP! Total: {}", exp.amount, player_exp.0);
        }
    }
}

// Handle spawning next enemy
pub fn next_enemy_spawn_system(
    mut commands: Commands,
    mut next_enemy_events: EventReader<NextEnemySpawnEvent>,
    mut game_state: ResMut<GameState>,
) {
    for spawn in next_enemy_events.read() {
        game_state.current_enemy_number = spawn.enemy_number;
        spawn_enemy(&mut commands, spawn.enemy_number);
        println!("Spawning enemy #{}", spawn.enemy_number);
    }
}

// Sync management stats to combat stats when upgrades happen  
pub fn sync_stats_system(
    mut player_query: Query<(
        &mut CombatAttack, 
        &mut CombatDefense, 
        &mut CombatSpeed, 
        &mut MaxHp,
        &mut CurrentHp,
        &BaseAttack, 
        &BaseDefense, 
        &BaseSpeed, 
        &BaseHp
    ), With<Player>>,
    stats: Query<(&crate::upgradeable_stat::UpgradeableStat, &crate::upgradeable_stat::CurrentValue), Changed<crate::upgradeable_stat::CurrentValue>>,
) {
    if let Ok((mut combat_attack, mut combat_defense, mut combat_speed, mut max_hp, mut current_hp, _base_attack, _base_defense, _base_speed, _base_hp)) = player_query.single_mut() {
        
        for (stat, current_value) in stats.iter() {
            let old_max_hp = max_hp.0.clone();
            
            match stat.name.as_str() {
                "HP" => {
                    max_hp.0 = current_value.0.clone();
                    // If max HP changed, update current HP
                    if max_hp.0 != old_max_hp {
                        current_hp.0 = max_hp.0.clone();
                    }
                }
                "Attack" => {
                    combat_attack.0 = current_value.0.clone();
                }
                "Defense" => {
                    combat_defense.0 = current_value.0.clone();
                }
                "Speed" => {
                    combat_speed.0 = current_value.0.clone();
                }
                _ => {} // Unknown stat name, ignore
            }
        }
    }
}