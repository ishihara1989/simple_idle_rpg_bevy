use bevy::prelude::*;
use too_big_float::BigFloat;
use crate::components::*;
use crate::events::*;

// Determine turn order based on speed
pub fn turn_order_system(
    time: Res<Time>,
    mut timer_query: Query<&mut CombatTimer>,
    player_query: Query<&CombatSpeed, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&CombatSpeed, (With<Enemy>, Without<Player>)>,
    mut turn_events: EventWriter<TurnStartEvent>,
    game_state: Res<GameState>,
) {
    if game_state.is_game_over {
        return;
    }

    let Ok(mut timer) = timer_query.get_single_mut() else { return };
    let Ok(player_speed) = player_query.get_single() else { return };
    let Ok(enemy_speed) = enemy_query.get_single() else { return };

    if timer.timer.tick(time.delta()).just_finished() {
        let player_speed_val = player_speed.0.to_f64().unwrap_or(1.0);
        let enemy_speed_val = enemy_speed.0.to_f64().unwrap_or(1.0);
        
        if player_speed_val >= enemy_speed_val {
            turn_events.send(TurnStartEvent { attacker: TurnAttacker::Player });
        } else {
            turn_events.send(TurnStartEvent { attacker: TurnAttacker::Enemy });
        }
    }
}

// Handle player attacks
pub fn player_attack_system(
    mut turn_events: EventReader<TurnStartEvent>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<(Entity, &CombatAttack), With<Player>>,
    enemy_query: Query<(Entity, &CombatDefense), With<Enemy>>,
) {
    for event in turn_events.read() {
        if event.attacker == TurnAttacker::Player {
            if let (Ok((player_entity, player_attack)), Ok((enemy_entity, enemy_defense))) = 
                (player_query.get_single(), enemy_query.get_single()) {
                    
                let damage = (player_attack.0.clone() - enemy_defense.0.clone()).max(BigFloat::from(1.0));
                
                attack_events.send(AttackEvent {
                    attacker: player_entity,
                    target: enemy_entity,
                    damage: damage.clone(),
                });
                
                println!("Player attacks for {} damage", damage);
            }
        }
    }
}

// Handle enemy attacks
pub fn enemy_attack_system(
    mut turn_events: EventReader<TurnStartEvent>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<(Entity, &CombatDefense), With<Player>>,
    enemy_query: Query<(Entity, &CombatAttack), With<Enemy>>,
) {
    for event in turn_events.read() {
        if event.attacker == TurnAttacker::Enemy {
            if let (Ok((player_entity, player_defense)), Ok((enemy_entity, enemy_attack))) = 
                (player_query.get_single(), enemy_query.get_single()) {
                    
                let damage = (enemy_attack.0.clone() - player_defense.0.clone()).max(BigFloat::from(1.0));
                
                attack_events.send(AttackEvent {
                    attacker: enemy_entity,
                    target: player_entity,
                    damage: damage.clone(),
                });
                
                println!("Enemy attacks for {} damage", damage);
            }
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
            let old_hp = current_hp.0.clone();
            current_hp.0 = (current_hp.0.clone() - attack.damage.clone()).max(BigFloat::from(0.0));
            
            println!("Target HP: {} -> {}", old_hp, current_hp.0);
            
            // Check for death
            if current_hp.0 <= BigFloat::from(0.0) {
                if player_query.get(attack.target).is_ok() {
                    death_events.send(DeathEvent {
                        entity: attack.target,
                        entity_type: DeathEntityType::Player,
                    });
                } else if enemy_query.get(attack.target).is_ok() {
                    death_events.send(DeathEvent {
                        entity: attack.target,
                        entity_type: DeathEntityType::Enemy,
                    });
                }
            }
        }
    }
}

// Events are now defined in crate::events::combat_events