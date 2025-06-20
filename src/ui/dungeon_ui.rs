use bevy::prelude::*;
use crate::{CombatState, AutomationConfig, DungeonButton, DungeonButtonText, AutoRetryButton, AutoRetryButtonText};
use crate::events::CombatStartEvent;

pub fn dungeon_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<DungeonButton>),
    >,
    mut button_text_query: Query<&mut Text, With<DungeonButtonText>>,
    combat_state: Res<CombatState>,
    automation_config: Res<AutomationConfig>,
    mut combat_start_events: EventWriter<CombatStartEvent>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if !combat_state.in_dungeon && !combat_state.is_game_over {
                    // Start new dungeon
                    combat_start_events.write(CombatStartEvent { is_retry: false });
                } else if combat_state.is_game_over {
                    // Manual retry
                    combat_start_events.write(CombatStartEvent { is_retry: true });
                }
                *background_color = BackgroundColor(Color::srgb(0.1, 0.5, 0.1));
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.25, 0.75, 0.25));
            }
            Interaction::None => {
                if combat_state.in_dungeon {
                    *background_color = BackgroundColor(Color::srgb(0.7, 0.2, 0.2));
                } else {
                    *background_color = BackgroundColor(Color::srgb(0.2, 0.7, 0.2));
                }
            }
        }
    }

    // Update button text based on combat state
    if let Ok(mut text) = button_text_query.single_mut() {
        if combat_state.is_game_over {
            text.0 = if automation_config.auto_retry_enabled { "Auto Retry..." } else { "Retry Dungeon" }.to_string();
        } else if combat_state.in_dungeon {
            text.0 = "In Dungeon...".to_string();
        } else {
            text.0 = "Enter Dungeon".to_string();
        }
    }
}

pub fn auto_retry_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<AutoRetryButton>),
    >,
    mut button_text_query: Query<&mut Text, With<AutoRetryButtonText>>,
    mut automation_config: ResMut<AutomationConfig>,
    combat_state: Res<CombatState>,
    mut combat_start_events: EventWriter<CombatStartEvent>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if automation_config.auto_retry_unlocked {
                    let was_enabled = automation_config.auto_retry_enabled;
                    automation_config.auto_retry_enabled = !automation_config.auto_retry_enabled;
                    
                    // If auto retry was just turned ON and we're in game over state, start combat
                    if !was_enabled && automation_config.auto_retry_enabled && combat_state.is_game_over {
                        combat_start_events.write(CombatStartEvent { is_retry: true });
                    }
                }
                *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.7));
            }
            Interaction::Hovered => {
                if automation_config.auto_retry_unlocked {
                    *background_color = BackgroundColor(Color::srgb(0.6, 0.6, 0.6));
                } else {
                    *background_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                }
            }
            Interaction::None => {
                if automation_config.auto_retry_unlocked {
                    if automation_config.auto_retry_enabled {
                        *background_color = BackgroundColor(Color::srgb(0.2, 0.7, 0.2));
                    } else {
                        *background_color = BackgroundColor(Color::srgb(0.7, 0.2, 0.2));
                    }
                } else {
                    *background_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                }
            }
        }
    }

    // Update button text based on unlock status and enabled state
    if let Ok(mut text) = button_text_query.single_mut() {
        if !automation_config.auto_retry_unlocked {
            text.0 = "Auto Retry: Locked".to_string();
        } else if automation_config.auto_retry_enabled {
            text.0 = "Auto Retry: ON".to_string();
        } else {
            text.0 = "Auto Retry: OFF".to_string();
        }
    }
}