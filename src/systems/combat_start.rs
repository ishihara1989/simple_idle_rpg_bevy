use bevy::prelude::*;
use crate::events::CombatStartEvent;
use crate::CombatState;

/// Centralized combat start system that handles all combat initiation
pub fn combat_start_system(
    mut combat_start_events: EventReader<CombatStartEvent>,
    mut combat_state: ResMut<CombatState>,
) {
    for event in combat_start_events.read() {
        println!(
            "Starting combat - {}",
            if event.is_retry { "Auto Retry" } else { "New Dungeon Entry" }
        );

        // Set combat state
        combat_state.is_game_over = false;
        combat_state.in_dungeon = true;
    }
}