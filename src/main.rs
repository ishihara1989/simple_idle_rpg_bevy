use bevy::prelude::*;
use simple_idle_rpg::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            is_game_over: false,
            current_enemy_number: 1,
            current_tab: GameTab::Combat,
        })
        // Add event types
        .add_event::<AttackEvent>()
        .add_event::<DeathEvent>()
        .add_event::<PlayerDeathEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_event::<ExpGainEvent>()
        .add_event::<NextEnemySpawnEvent>()
        .add_event::<CombatEndEvent>()
        // Startup systems
        .add_systems(Startup, (
            player_init_system,
            setup_ui,
        ))
        // Main game loop systems
        .add_systems(Update, (
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
            
            // Upgrade and sync systems
            upgradeable_stat_upgrade_system,
            update_current_value_on_change,
            (
                hp_sync_system,
                attack_sync_system,
                defense_sync_system,
                speed_sync_system,
            ).after(upgradeable_stat_upgrade_system),
            
            // UI systems
            update_ui_system,
            tab_button_system,
        ))
        .run();
}

