use bevy::prelude::*;
use crate::{
    AttackEvent, DeathEvent, PlayerDeathEvent, EnemyDeathEvent, 
    ExpGainEvent, NextEnemySpawnEvent, CombatEndEvent, CombatStartEvent, CombatState,
    combat_init_system, combat_start_system, attack_cooldown_system, player_attack_system, 
    enemy_attack_system, damage_application_system, death_detection_system,
    enemy_death_system, player_death_system, exp_gain_system, next_enemy_spawn_system
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CombatState {
                is_game_over: false,
                in_dungeon: false,
            })
            // Add combat events
            .add_event::<CombatStartEvent>()
            .add_event::<AttackEvent>()
            .add_event::<DeathEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<EnemyDeathEvent>()
            .add_event::<ExpGainEvent>()
            .add_event::<NextEnemySpawnEvent>()
            .add_event::<CombatEndEvent>()
            // Add combat systems
            .add_systems(Update, (
                // Combat control systems
                combat_start_system,
                
                // Initialization (only runs when needed)
                combat_init_system,
                
                // Real-time combat systems
                attack_cooldown_system,
                (player_attack_system, enemy_attack_system),
                damage_application_system,
                
                // Combat end systems
                death_detection_system,
                (enemy_death_system, player_death_system),
                exp_gain_system,
                next_enemy_spawn_system,
            ));
    }
}