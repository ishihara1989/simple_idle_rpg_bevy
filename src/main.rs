use bevy::prelude::*;
use too_big_float::BigFloat;

mod upgradeable_stat;
use upgradeable_stat::*;

#[derive(Component)]
struct Player {
    hp: BigFloat,
    max_hp: BigFloat,
    attack: BigFloat,
    defense: BigFloat,
    speed: BigFloat,
    exp: BigFloat,
    level: u32,
    rebirth_points: BigFloat,
    stat_upgrade_costs: StatUpgradeCosts,
}

#[derive(Component)]
struct Enemy {
    hp: BigFloat,
    max_hp: BigFloat,
    attack: BigFloat,
    defense: BigFloat,
    speed: BigFloat,
    exp_reward: BigFloat,
    enemy_number: u32,
}

#[derive(Component)]
struct StatUpgradeCosts {
    hp_cost: BigFloat,
    attack_cost: BigFloat,
    defense_cost: BigFloat,
    speed_cost: BigFloat,
}

#[derive(Component)]
struct CombatTimer {
    timer: Timer,
}

#[derive(Resource)]
struct GameState {
    is_game_over: bool,
    current_enemy_number: u32,
    current_tab: GameTab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameTab {
    Combat,
    Rebirth,
}

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct CombatText;

#[derive(Component)]
struct TabButton {
    tab: GameTab,
}

#[derive(Component)]
struct TabContent {
    tab: GameTab,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            is_game_over: false,
            current_enemy_number: 1,
            current_tab: GameTab::Combat,
        })
        .add_systems(Startup, (setup_game, setup_ui))
        .add_systems(Update, (
            combat_system,
            handle_death,
            spawn_next_enemy,
            upgradeable_stat_upgrade_system,
            update_current_value_on_change,
            sync_player_stats_system.after(upgradeable_stat_upgrade_system),
            update_ui_system,
            tab_button_system,
        ))
        .run();
}

fn setup_game(mut commands: Commands, game_state: ResMut<GameState>) {
    let base_hp = BigFloat::from(100.0);
    let base_attack = BigFloat::from(10.0);
    let base_defense = BigFloat::from(5.0);
    let base_speed = BigFloat::from(1.0);
    let base_cost = BigFloat::from(10.0);

    commands.spawn(Player {
        hp: base_hp.clone(),
        max_hp: base_hp.clone(),
        attack: base_attack.clone(),
        defense: base_defense.clone(),
        speed: base_speed.clone(),
        exp: BigFloat::from(0.0),
        level: 1,
        rebirth_points: BigFloat::from(0.0),
        stat_upgrade_costs: StatUpgradeCosts {
            hp_cost: base_cost.clone(),
            attack_cost: base_cost.clone(),
            defense_cost: base_cost.clone(),
            speed_cost: base_cost.clone(),
        },
    });

    // 新しい個別コンポーネント設計のUpgradeableStatを作成
    commands.spawn(UpgradeableStatBundle::new("HP", base_hp, base_cost.clone(), 1.15, 1.3));
    commands.spawn(UpgradeableStatBundle::new("Attack", base_attack, base_cost.clone(), 1.15, 1.3));
    commands.spawn(UpgradeableStatBundle::new("Defense", base_defense, base_cost.clone(), 1.15, 1.3));
    commands.spawn(UpgradeableStatBundle::new("Speed", base_speed, base_cost, 1.15, 1.3));

    spawn_enemy(&mut commands, game_state.current_enemy_number);

    commands.spawn(CombatTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        )).with_children(|parent| {
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                BorderColor(Color::WHITE),
                BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                TabButton { tab: GameTab::Combat },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Combat"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::WHITE),
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                TabButton { tab: GameTab::Rebirth },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Rebirth"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });

        parent.spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        )).with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                TabContent { tab: GameTab::Combat },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Combat Stats"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Loading..."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    StatsText,
                ));
                
                parent.spawn((
                    Text::new("Combat Log"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Fight starting..."),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 0.0)),
                    CombatText,
                ));
            });
            
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    display: Display::None,
                    ..default()
                },
                TabContent { tab: GameTab::Rebirth },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Rebirth System"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Rebirth features coming soon..."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

fn sync_player_stats_system(
    mut player_query: Query<&mut Player>,
    stats: Query<(&UpgradeableStat, &CurrentValue, &UpgradeCost)>,
) {
    let Ok(mut player) = player_query.single_mut() else { return };
    
    for (stat, current_value, upgrade_cost) in stats.iter() {
        let old_max_hp = player.max_hp.clone();
        
        // 名前に基づいてプレイヤーのステータスを更新
        match stat.name.as_str() {
            "HP" => {
                player.max_hp = current_value.0.clone();
                player.stat_upgrade_costs.hp_cost = upgrade_cost.0.clone();
                // HPの最大値が変わった場合、現在のHPも更新
                if player.max_hp != old_max_hp {
                    player.hp = player.max_hp.clone();
                }
            }
            "Attack" => {
                player.attack = current_value.0.clone();
                player.stat_upgrade_costs.attack_cost = upgrade_cost.0.clone();
            }
            "Defense" => {
                player.defense = current_value.0.clone();
                player.stat_upgrade_costs.defense_cost = upgrade_cost.0.clone();
            }
            "Speed" => {
                player.speed = current_value.0.clone();
                player.stat_upgrade_costs.speed_cost = upgrade_cost.0.clone();
            }
            _ => {} // 未知のステータス名は無視
        }
    }
}

fn spawn_enemy(commands: &mut Commands, enemy_number: u32) {
    let base_hp = calculate_exponential_growth(BigFloat::from(80.0), 1.5, enemy_number);
    let base_attack = calculate_exponential_growth(BigFloat::from(8.0), 1.3, enemy_number);
    let base_defense = calculate_exponential_growth(BigFloat::from(3.0), 1.3, enemy_number);
    let base_speed = calculate_exponential_growth(BigFloat::from(0.8), 1.1, enemy_number);
    let base_exp = calculate_exponential_growth(BigFloat::from(5.0), 1.15, enemy_number);

    commands.spawn(Enemy {
        hp: base_hp.clone(),
        max_hp: base_hp,
        attack: base_attack,
        defense: base_defense,
        speed: base_speed,
        exp_reward: base_exp,
        enemy_number,
    });
}

fn combat_system(
    time: Res<Time>,
    mut timer_query: Query<&mut CombatTimer>,
    mut player_query: Query<&mut Player>,
    mut enemy_query: Query<&mut Enemy>,
    game_state: Res<GameState>,
) {
    if game_state.is_game_over {
        return;
    }

    let Ok(mut timer) = timer_query.single_mut() else { return };
    let Ok(mut player) = player_query.single_mut() else { return };
    let Ok(mut enemy) = enemy_query.single_mut() else { return };

    if timer.timer.tick(time.delta()).just_finished() {
        let player_speed = player.speed.to_f64().unwrap_or(1.0);
        let enemy_speed = enemy.speed.to_f64().unwrap_or(1.0);
        
        if player_speed >= enemy_speed {
            let damage = (player.attack.clone() - enemy.defense.clone()).max(BigFloat::from(1.0));
            enemy.hp = (enemy.hp.clone() - damage).max(BigFloat::from(0.0));
            
            if enemy.hp <= BigFloat::from(0.0) {
                player.exp += enemy.exp_reward.clone();
                println!("Enemy {} defeated! Gained {} EXP", enemy.enemy_number, enemy.exp_reward);
                return;
            }
        }
        
        let damage = (enemy.attack.clone() - player.defense.clone()).max(BigFloat::from(1.0));
        player.hp = (player.hp.clone() - damage).max(BigFloat::from(0.0));
        
        println!("Player HP: {}, Enemy HP: {}", player.hp, enemy.hp);
    }
}

fn handle_death(
    mut commands: Commands,
    player_query: Query<(Entity, &Player)>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut game_state: ResMut<GameState>,
) {
    if let Ok((player_entity, player)) = player_query.single() {
        if player.hp <= BigFloat::from(0.0) {
            println!("Game Over! Starting rebirth...");
            
            let rebirth_gain = BigFloat::from(game_state.current_enemy_number as f64);
            println!("Gained {} rebirth points", rebirth_gain);
            
            commands.entity(player_entity).despawn();
            for enemy_entity in enemy_query.iter() {
                commands.entity(enemy_entity).despawn();
            }
            
            game_state.current_enemy_number = 1;
            
            rebirth_player(&mut commands, rebirth_gain);
            spawn_enemy(&mut commands, 1);
            
            game_state.is_game_over = false;
        }
    }
}

fn spawn_next_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Enemy)>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.is_game_over {
        return;
    }

    if let Ok((enemy_entity, enemy)) = enemy_query.single() {
        if enemy.hp <= BigFloat::from(0.0) {
            commands.entity(enemy_entity).despawn();
            game_state.current_enemy_number += 1;
            spawn_enemy(&mut commands, game_state.current_enemy_number);
            println!("Spawning enemy #{}", game_state.current_enemy_number);
        }
    }
}

fn rebirth_player(commands: &mut Commands, additional_rebirth_points: BigFloat) {
    let rebirth_bonus = additional_rebirth_points.clone() * BigFloat::from(0.1) + BigFloat::from(1.0);
    
    let base_hp = BigFloat::from(100.0) * rebirth_bonus.clone();
    let base_attack = BigFloat::from(10.0) * rebirth_bonus.clone();
    let base_defense = BigFloat::from(5.0) * rebirth_bonus.clone();
    let base_speed = BigFloat::from(1.0) * rebirth_bonus;
    let base_cost = BigFloat::from(10.0) / (additional_rebirth_points.clone() * BigFloat::from(0.05) + BigFloat::from(1.0));

    println!("Reborn with enhanced stats! Rebirth bonus: {}x", additional_rebirth_points.clone() * BigFloat::from(0.1) + BigFloat::from(1.0));

    commands.spawn(Player {
        hp: base_hp.clone(),
        max_hp: base_hp.clone(),
        attack: base_attack.clone(),
        defense: base_defense.clone(),
        speed: base_speed.clone(),
        exp: BigFloat::from(0.0),
        level: 1,
        rebirth_points: additional_rebirth_points,
        stat_upgrade_costs: StatUpgradeCosts {
            hp_cost: base_cost.clone(),
            attack_cost: base_cost.clone(),
            defense_cost: base_cost.clone(),
            speed_cost: base_cost.clone(),
        },
    });

    // 新しい個別コンポーネント設計のUpgradeableStatを作成
    commands.spawn(UpgradeableStatBundle::new("HP", base_hp, base_cost.clone(), 1.15, 1.3));
    commands.spawn(UpgradeableStatBundle::new("Attack", base_attack, base_cost.clone(), 1.15, 1.3));
    commands.spawn(UpgradeableStatBundle::new("Defense", base_defense, base_cost.clone(), 1.15, 1.3));
    commands.spawn(UpgradeableStatBundle::new("Speed", base_speed, base_cost, 1.15, 1.3));

    commands.spawn(CombatTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

fn update_ui_system(
    player_query: Query<&Player>,
    enemy_query: Query<&Enemy>,
    mut stats_text_query: Query<&mut Text, (With<StatsText>, Without<CombatText>)>,
    mut combat_text_query: Query<&mut Text, (With<CombatText>, Without<StatsText>)>,
) {
    if let Ok(player) = player_query.single() {
        if let Ok(mut stats_text) = stats_text_query.single_mut() {
            let stats_info = format!(
                "Player Stats:\nHP: {:.2}\nAttack: {:.2}\nDefense: {:.2}\nSpeed: {:.2}\nEXP: {:.2}\n\nUpgrade Costs:\nHP: {:.2}\nAttack: {:.2}\nDefense: {:.2}\nSpeed: {:.2}",
                player.hp.to_f64().unwrap_or(0.0),
                player.attack.to_f64().unwrap_or(0.0),
                player.defense.to_f64().unwrap_or(0.0),
                player.speed.to_f64().unwrap_or(0.0),
                player.exp.to_f64().unwrap_or(0.0),
                player.stat_upgrade_costs.hp_cost.to_f64().unwrap_or(0.0),
                player.stat_upgrade_costs.attack_cost.to_f64().unwrap_or(0.0),
                player.stat_upgrade_costs.defense_cost.to_f64().unwrap_or(0.0),
                player.stat_upgrade_costs.speed_cost.to_f64().unwrap_or(0.0),
            );
            **stats_text = stats_info;
        }
    }

    if let Ok(enemy) = enemy_query.single() {
        if let Ok(mut combat_text) = combat_text_query.single_mut() {
            let combat_info = format!(
                "Enemy #{}\nEnemy HP: {:.2}\nEnemy Attack: {:.2}\nEnemy Defense: {:.2}\nEnemy Speed: {:.2}\nEXP Reward: {:.2}",
                enemy.enemy_number,
                enemy.hp.to_f64().unwrap_or(0.0),
                enemy.attack.to_f64().unwrap_or(0.0),
                enemy.defense.to_f64().unwrap_or(0.0),
                enemy.speed.to_f64().unwrap_or(0.0),
                enemy.exp_reward.to_f64().unwrap_or(0.0),
            );
            **combat_text = combat_info;
        }
    }
}

fn tab_button_system(
    mut interaction_query: Query<
        (&Interaction, &TabButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut tab_content_query: Query<(&mut Node, &TabContent)>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, tab_button, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_state.current_tab = tab_button.tab;
                
                for (mut node, tab_content) in &mut tab_content_query {
                    if tab_content.tab == game_state.current_tab {
                        node.display = Display::Flex;
                    } else {
                        node.display = Display::None;
                    }
                }
                
                *background_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.45, 0.45, 0.45));
            }
            Interaction::None => {
                *background_color = if tab_button.tab == game_state.current_tab {
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4))
                } else {
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3))
                };
            }
        }
    }
}