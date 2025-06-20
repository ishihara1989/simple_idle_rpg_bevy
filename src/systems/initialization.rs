use bevy::prelude::*;
use too_big_float::BigFloat;
use crate::components::*;
use crate::{StartupConfig, GameProgress};
// All components are now imported via crate::components::*

// Initialize player with base management stats
pub fn player_init_system(
    mut commands: Commands,
    config: Res<StartupConfig>,
) {
    let base_hp = BigFloat::from(100.0);
    let base_attack = BigFloat::from(10.0);
    let base_defense = BigFloat::from(5.0);
    let base_speed = BigFloat::from(1.0);
    let base_cost = BigFloat::from(10.0);

    // Create player entity with management stats (using config values)
    commands.spawn((
        Player,
        BaseHp(base_hp.clone()),
        BaseAttack(base_attack.clone()),
        BaseDefense(base_defense.clone()),
        BaseSpeed(base_speed.clone()),
        Experience(BigFloat::from(config.experience as f64)),
        Level(config.level),
        RebirthPoints(BigFloat::from(0.0)),
    ));

    // Create upgradeable stat entities using typed bundles with config values
    commands.spawn((
        UpgradeableHp,
        Player,
        CurrentValue(base_hp.clone()),
        BaseValue(base_hp.clone()),
        UpgradeLevel(config.hp_level),
        UpgradeCost(base_cost.clone()),
        UpgradeMultiplier(1.15),
        CostMultiplier(1.3),
    ));

    commands.spawn((
        UpgradeableAttack,
        Player,
        CurrentValue(base_attack.clone()),
        BaseValue(base_attack.clone()),
        UpgradeLevel(config.attack_level),
        UpgradeCost(base_cost.clone()),
        UpgradeMultiplier(1.15),
        CostMultiplier(1.3),
    ));

    commands.spawn((
        UpgradeableDefense,
        Player,
        CurrentValue(base_defense.clone()),
        BaseValue(base_defense.clone()),
        UpgradeLevel(config.defense_level),
        UpgradeCost(base_cost.clone()),
        UpgradeMultiplier(1.15),
        CostMultiplier(1.3),
    ));

    commands.spawn((
        UpgradeableSpeed,
        Player,
        CurrentValue(base_speed),
        BaseValue(base_speed),
        UpgradeLevel(config.speed_level),
        UpgradeCost(base_cost),
        UpgradeMultiplier(1.15),
        CostMultiplier(1.3),
    ));
}

// Initialize combat by copying management stats to combat stats
pub fn combat_init_system(
    mut commands: Commands,
    player_query: Query<(Entity, &BaseHp, &BaseAttack, &BaseDefense, &BaseSpeed), (With<Player>, Without<CurrentHp>)>,
    game_progress: Res<GameProgress>,
) {
    if let Ok((player_entity, base_hp, base_attack, base_defense, base_speed)) = player_query.single() {
        // Add combat stats to player entity
        commands.entity(player_entity).insert((
            CurrentHp(base_hp.0),
            MaxHp(base_hp.0),
            CombatAttack(base_attack.0),
            CombatDefense(base_defense.0),
            CombatSpeed(base_speed.0),
            AttackCooldown(0.0), // Start ready to attack
        ));

        // Spawn initial enemy
        spawn_enemy(&mut commands, game_progress.current_enemy_number);

        // Add combat timer
        commands.spawn(CombatTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });
    }
}

// Helper function to spawn enemies (keeping existing logic)
pub fn spawn_enemy(commands: &mut Commands, enemy_number: u32) {
    let base_hp = calculate_exponential_growth(BigFloat::from(20.0), 1.5, enemy_number);
    let base_attack = calculate_exponential_growth(BigFloat::from(3.0), 1.3, enemy_number);
    let base_defense = calculate_exponential_growth(BigFloat::from(2.0), 1.3, enemy_number);
    let base_speed = calculate_exponential_growth(BigFloat::from(0.8), 1.1, enemy_number);
    let base_exp = calculate_exponential_growth(BigFloat::from(5.0), 1.15, enemy_number);

    commands.spawn((
        Enemy,
        CurrentHp(base_hp),
        MaxHp(base_hp),
        CombatAttack(base_attack),
        CombatDefense(base_defense),
        CombatSpeed(base_speed),
        ExpReward(base_exp),
        EnemyNumber(enemy_number),
        AttackCooldown(0.0), // Start ready to attack
    ));
}

// Rebirth system for restarting with enhanced stats
pub fn rebirth_player_system(
    commands: &mut Commands,
    additional_rebirth_points: BigFloat,
) {
    let rebirth_bonus = additional_rebirth_points * BigFloat::from(0.1) + BigFloat::from(1.0);
    
    let base_hp = BigFloat::from(100.0) * rebirth_bonus;
    let base_attack = BigFloat::from(10.0) * rebirth_bonus;
    let base_defense = BigFloat::from(5.0) * rebirth_bonus;
    let base_speed = BigFloat::from(1.0) * rebirth_bonus;
    let base_cost = BigFloat::from(10.0) / (additional_rebirth_points * BigFloat::from(0.05) + BigFloat::from(1.0));

    println!("Reborn with enhanced stats! Rebirth bonus: {}x", rebirth_bonus);

    // Create new player with enhanced base stats
    commands.spawn((
        Player,
        BaseHp(base_hp),
        BaseAttack(base_attack),
        BaseDefense(base_defense),
        BaseSpeed(base_speed),
        Experience(BigFloat::from(0.0)),
        Level(1),
        RebirthPoints(additional_rebirth_points),
    ));

    // Create new upgradeable stat entities using typed bundles
    commands.spawn(UpgradeableHpBundle::new(base_hp, base_cost, 1.15, 1.3));
    commands.spawn(UpgradeableAttackBundle::new(base_attack, base_cost, 1.15, 1.3));
    commands.spawn(UpgradeableDefenseBundle::new(base_defense, base_cost, 1.15, 1.3));
    commands.spawn(UpgradeableSpeedBundle::new(base_speed, base_cost, 1.15, 1.3));

    // Add combat timer
    commands.spawn(CombatTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}