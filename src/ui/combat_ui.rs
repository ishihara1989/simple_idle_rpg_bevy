use bevy::prelude::*;
use crate::{
    Player, Enemy, Experience, CurrentHp, CombatAttack, CombatDefense, CombatSpeed,
    EnemyNumber, ExpReward, StatsText, CombatText
};

pub fn update_ui_system(
    player_query: Query<(&Experience, &CurrentHp, &CombatAttack, &CombatDefense, &CombatSpeed), With<Player>>,
    enemy_query: Query<(&EnemyNumber, &CurrentHp, &CombatAttack, &CombatDefense, &CombatSpeed, &ExpReward), With<Enemy>>,
    mut stats_text_query: Query<&mut Text, (With<StatsText>, Without<CombatText>)>,
    mut combat_text_query: Query<&mut Text, (With<CombatText>, Without<StatsText>)>,
) {
    // Update player stats display
    if let Ok((exp, hp, attack, defense, speed)) = player_query.single() {
        if let Ok(mut stats_text) = stats_text_query.single_mut() {
            let stats_info = format!(
                "Player Stats:\nHP: {:.2}\nAttack: {:.2}\nDefense: {:.2}\nSpeed: {:.2}\nEXP: {:.2}",
                hp.0.to_f64().unwrap_or(0.0),
                attack.0.to_f64().unwrap_or(0.0),
                defense.0.to_f64().unwrap_or(0.0),
                speed.0.to_f64().unwrap_or(0.0),
                exp.0.to_f64().unwrap_or(0.0),
            );
            **stats_text = stats_info;
        }
    }

    // Update enemy stats display
    if let Ok((enemy_number, hp, attack, defense, speed, exp_reward)) = enemy_query.single() {
        if let Ok(mut combat_text) = combat_text_query.single_mut() {
            let combat_info = format!(
                "Enemy #{}\nEnemy HP: {:.2}\nEnemy Attack: {:.2}\nEnemy Defense: {:.2}\nEnemy Speed: {:.2}\nEXP Reward: {:.2}",
                enemy_number.0,
                hp.0.to_f64().unwrap_or(0.0),
                attack.0.to_f64().unwrap_or(0.0),
                defense.0.to_f64().unwrap_or(0.0),
                speed.0.to_f64().unwrap_or(0.0),
                exp_reward.0.to_f64().unwrap_or(0.0),
            );
            **combat_text = combat_info;
        }
    }
}