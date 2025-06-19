use bevy::prelude::*;
use crate::{setup_ui, update_ui_system, tab_button_system};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_ui_system,
                tab_button_system,
            ));
    }
}