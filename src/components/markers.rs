use bevy::prelude::*;

// Marker components for identification
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// UI marker components (kept from original)
#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct CombatText;

#[derive(Component)]
pub struct TabButton {
    pub tab: GameTab,
}

#[derive(Component)]
pub struct TabContent {
    pub tab: GameTab,
}

// Game state
#[derive(Resource)]
pub struct GameState {
    pub is_game_over: bool,
    pub current_enemy_number: u32,
    pub current_tab: GameTab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameTab {
    Combat,
    Rebirth,
}