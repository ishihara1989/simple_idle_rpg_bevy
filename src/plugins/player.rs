use bevy::prelude::*;
use crate::{player_init_system, GameState, GameTab};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState {
                is_game_over: false,
                current_enemy_number: 1,
                current_tab: GameTab::Combat,
            })
            .add_systems(Startup, player_init_system);
    }
}