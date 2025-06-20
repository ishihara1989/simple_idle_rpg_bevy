use bevy::prelude::*;
use crate::{player_init_system, GameProgress};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameProgress {
                current_enemy_number: 1,
                has_died_once: false,
            })
            .add_systems(Startup, player_init_system);
    }
}