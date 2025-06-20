use bevy::prelude::*;
use crate::{
    Player, Enemy, 
    Level, Experience,
    CurrentHp, MaxHp, CombatAttack, CombatDefense, CombatSpeed,
    UpgradeLevel, UpgradeableHp, UpgradeableAttack, UpgradeableDefense, UpgradeableSpeed,
    EnemyNumber, StartupConfig, AutomationConfig,
};
use std::time::Duration;

pub struct BalanceCheckPlugin;

impl Plugin for BalanceCheckPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AutomationConfig {
                auto_retry_unlocked: true,  // Always unlocked in balance check mode
                auto_retry_enabled: true,   // Always enabled in balance check mode
            })
            .add_systems(Startup, setup_balance_check_timer)
            .add_systems(Update, (
                balance_check_output_system,
                auto_shutdown_system,
            ));
    }
}

#[derive(Resource)]
pub struct BalanceCheckTimer {
    timer: Timer,
    output_count: u32,
    max_outputs: u32,
}

fn setup_balance_check_timer(
    mut commands: Commands,
    config: Res<StartupConfig>,
) {
    commands.insert_resource(BalanceCheckTimer {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        output_count: 0,
        max_outputs: config.duration as u32,
    });
}

fn balance_check_output_system(
    mut timer: ResMut<BalanceCheckTimer>,
    time: Res<Time>,
    player_query: Query<(
        Option<&Level>,
        Option<&Experience>,
        Option<&CurrentHp>,
        Option<&MaxHp>,
        Option<&CombatAttack>,
        Option<&CombatDefense>,
        Option<&CombatSpeed>,
    ), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(
        Option<&CurrentHp>,
        Option<&MaxHp>,
        Option<&CombatAttack>,
        Option<&CombatDefense>,
        Option<&CombatSpeed>,
        Option<&EnemyNumber>,
    ), (With<Enemy>, Without<Player>)>,
    upgradeable_hp_query: Query<&UpgradeLevel, (With<UpgradeableHp>, With<Player>)>,
    upgradeable_attack_query: Query<&UpgradeLevel, (With<UpgradeableAttack>, With<Player>)>,
    upgradeable_defense_query: Query<&UpgradeLevel, (With<UpgradeableDefense>, With<Player>)>,
    upgradeable_speed_query: Query<&UpgradeLevel, (With<UpgradeableSpeed>, With<Player>)>,
) {
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        timer.output_count += 1;
        
        println!("\n=== Balance Check Report #{} ===", timer.output_count);
        
        // プレイヤー情報出力
        if let Ok((level, experience, current_hp, max_hp, attack, defense, speed)) = player_query.single() {
            println!("Player Status:");
            if let Some(level) = level {
                println!("  Level: {}", level.0);
            }
            if let Some(exp) = experience {
                println!("  Experience: {}", exp.0);
            }
            if let (Some(current), Some(max)) = (current_hp, max_hp) {
                println!("  HP: {}/{}", current.0, max.0);
            }
            if let Some(att) = attack {
                println!("  Attack: {}", att.0);
            }
            if let Some(def) = defense {
                println!("  Defense: {}", def.0);
            }
            if let Some(spd) = speed {
                println!("  Speed: {}", spd.0);
            }
            
            println!("  Upgradeable Stats:");
            if let Ok(hp_level) = upgradeable_hp_query.single() {
                println!("    HP Level: {}", hp_level.0);
            }
            if let Ok(att_level) = upgradeable_attack_query.single() {
                println!("    Attack Level: {}", att_level.0);
            }
            if let Ok(def_level) = upgradeable_defense_query.single() {
                println!("    Defense Level: {}", def_level.0);
            }
            if let Ok(spd_level) = upgradeable_speed_query.single() {
                println!("    Speed Level: {}", spd_level.0);
            }
        }
        
        // 敵情報出力
        let enemy_count = enemy_query.iter().count();
        println!("Enemy Count: {}", enemy_count);
        
        if enemy_count > 0 {
            println!("Enemy Status:");
            for (i, (current_hp, max_hp, attack, defense, speed, enemy_num)) in enemy_query.iter().enumerate() {
                let enemy_id = if let Some(num) = enemy_num { num.0 } else { i as u32 + 1 };
                print!("  Enemy {}: ", enemy_id);
                
                if let (Some(current), Some(max)) = (current_hp, max_hp) {
                    print!("HP {}/{}, ", current.0, max.0);
                }
                if let Some(att) = attack {
                    print!("Attack {}, ", att.0);
                }
                if let Some(def) = defense {
                    print!("Defense {}, ", def.0);
                }
                if let Some(spd) = speed {
                    print!("Speed {}", spd.0);
                }
                println!();
            }
        }
        
        println!("=== End Report ===\n");
    }
}

fn auto_shutdown_system(
    timer: Res<BalanceCheckTimer>,
    mut exit: EventWriter<AppExit>,
) {
    if timer.output_count >= timer.max_outputs {
        println!("Balance check completed. Shutting down...");
        exit.write(AppExit::Success);
    }
}