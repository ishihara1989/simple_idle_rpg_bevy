use bevy::prelude::*;
use crate::{setup_ui, update_ui_system, tab_button_system, dungeon_button_system, auto_retry_button_system, UIState, AutomationConfig, GameTab};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UIState {
                current_tab: GameTab::Combat,
            })
            .insert_resource(AutomationConfig {
                auto_retry_unlocked: false,
                auto_retry_enabled: false,
            })
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_ui_system,
                tab_button_system,
                dungeon_button_system,
                auto_retry_button_system,
            ));
    }
}