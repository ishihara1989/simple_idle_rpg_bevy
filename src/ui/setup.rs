use bevy::prelude::*;
use crate::{GameTab, TabButton, TabContent, StatsText, CombatText, DungeonButton, DungeonButtonText, AutoRetryButton, AutoRetryButtonText};

pub fn setup_ui(mut commands: Commands) {
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
                    margin: UiRect::bottom(Val::Px(10.0)),
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

            // Automation tab button
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
                TabButton { tab: GameTab::Automation },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Automation"),
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

                // Dungeon control buttons
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.7, 0.2)),
                    BorderColor(Color::WHITE),
                    DungeonButton,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Enter Dungeon"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::WHITE),
                        DungeonButtonText,
                    ));
                });
                
                parent.spawn((
                    Text::new("Combat Log"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Text::new("Click 'Enter Dungeon' to start combat"),
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
            
            // Automation tab content
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    display: Display::None,
                    ..default()
                },
                TabContent { tab: GameTab::Automation },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Automation Settings"),
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    BorderColor(Color::WHITE),
                    AutoRetryButton,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Auto Retry: Locked"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::WHITE),
                        AutoRetryButtonText,
                    ));
                });
            });
        });
    });
}