use bevy::prelude::*;
use too_big_float::BigFloat;
use crate::components::*;
use crate::events::*;
use crate::CombatState;

// Real-time cooldown system - reduces cooldowns based on speed and time
pub fn attack_cooldown_system(
    time: Res<Time>,
    mut cooldown_query: Query<(&mut AttackCooldown, &CombatSpeed)>,
    combat_state: Res<CombatState>,
) {
    if combat_state.is_game_over || !combat_state.in_dungeon {
        return;
    }

    let delta_ms = time.delta().as_millis() as f32;
    
    for (mut cooldown, speed) in cooldown_query.iter_mut() {
        if cooldown.0 > 0.0 {
            // Cooldown reduction is proportional to speed
            // Higher speed = faster cooldown reduction
            let speed_multiplier = speed.0.to_f64().unwrap_or(1.0) as f32;
            let reduction = delta_ms * speed_multiplier;
            cooldown.0 = (cooldown.0 - reduction).max(0.0);
        }
    }
}

// Shared helper function for attack execution logic
fn execute_attack_if_ready(
    attacker_entity: Entity,
    attack: &CombatAttack,
    speed: &CombatSpeed,
    cooldown: &mut AttackCooldown,
    target_entity: Entity,
    target_defense: &CombatDefense,
    attack_events: &mut EventWriter<AttackEvent>,
    attacker_name: &str,
) -> bool {
    if cooldown.0 <= 0.0 {
        let damage = (attack.0 - target_defense.0).max(BigFloat::from(1.0));
        
        attack_events.write(AttackEvent {
            attacker: attacker_entity,
            target: target_entity,
            damage,
        });
        
        // Calculate base attack time (1000ms) adjusted by speed
        let speed_value = speed.0.to_f64().unwrap_or(1.0) as f32;
        let base_attack_time = 1000.0; // 1 second base
        cooldown.0 = base_attack_time / speed_value;
        
        println!("{} attacks for {} damage (cooldown: {}ms)", attacker_name, damage, cooldown.0);
        true
    } else {
        false
    }
}

// Player attack system - handles only player attacks
pub fn player_attack_system(
    mut attack_events: EventWriter<AttackEvent>,
    mut player_query: Query<(Entity, &CombatAttack, &CombatSpeed, &mut AttackCooldown), (With<Player>, Without<Enemy>)>,
    target_query: Query<(Entity, &CombatDefense), (With<Enemy>, Without<Player>)>,
    combat_state: Res<CombatState>,
) {
    if combat_state.is_game_over || !combat_state.in_dungeon {
        return;
    }

    if let Ok((player_entity, player_attack, player_speed, mut player_cooldown)) = player_query.single_mut() {
        if let Ok((enemy_entity, enemy_defense)) = target_query.single() {
            execute_attack_if_ready(
                player_entity,
                player_attack,
                player_speed,
                &mut player_cooldown,
                enemy_entity,
                enemy_defense,
                &mut attack_events,
                "Player",
            );
        }
    }
}

// Enemy attack system - handles only enemy attacks
pub fn enemy_attack_system(
    mut attack_events: EventWriter<AttackEvent>,
    mut enemy_query: Query<(Entity, &CombatAttack, &CombatSpeed, &mut AttackCooldown), (With<Enemy>, Without<Player>)>,
    target_query: Query<(Entity, &CombatDefense), (With<Player>, Without<Enemy>)>,
    combat_state: Res<CombatState>,
) {
    if combat_state.is_game_over || !combat_state.in_dungeon {
        return;
    }

    if let Ok((enemy_entity, enemy_attack, enemy_speed, mut enemy_cooldown)) = enemy_query.single_mut() {
        if let Ok((player_entity, player_defense)) = target_query.single() {
            execute_attack_if_ready(
                enemy_entity,
                enemy_attack,
                enemy_speed,
                &mut enemy_cooldown,
                player_entity,
                player_defense,
                &mut attack_events,
                "Enemy",
            );
        }
    }
}

// Apply damage to targets
pub fn damage_application_system(
    mut attack_events: EventReader<AttackEvent>,
    mut hp_query: Query<&mut CurrentHp>,
    mut death_events: EventWriter<DeathEvent>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for attack in attack_events.read() {
        if let Ok(mut current_hp) = hp_query.get_mut(attack.target) {
            let old_hp = current_hp.0;
            current_hp.0 = (current_hp.0 - attack.damage).max(BigFloat::from(0.0));
            
            println!("Target HP: {} -> {}", old_hp, current_hp.0);
            
            // Check for death
            if current_hp.0 <= BigFloat::from(0.0) {
                if player_query.get(attack.target).is_ok() {
                    death_events.write(DeathEvent {
                        entity: attack.target,
                        entity_type: DeathEntityType::Player,
                    });
                } else if enemy_query.get(attack.target).is_ok() {
                    death_events.write(DeathEvent {
                        entity: attack.target,
                        entity_type: DeathEntityType::Enemy,
                    });
                }
            }
        }
    }
}