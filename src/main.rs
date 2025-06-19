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
            real_time_attack_system,
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

// UI setup function (kept from original but simplified)
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
        // Left sidebar with tab buttons
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
            // Combat tab button
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
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            // Rebirth tab button
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
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // Main content area
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
            // Combat tab content
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
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Loading..."),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                    StatsText,
                ));
                
                parent.spawn((
                    Text::new("Combat Log"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Fight starting..."),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(1.0, 1.0, 0.0)),
                    CombatText,
                ));
            });
            
            // Rebirth tab content
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
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Rebirth features coming soon..."),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

// UI update system (simplified version from original)
fn update_ui_system(
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

// Tab button system (kept from original)
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