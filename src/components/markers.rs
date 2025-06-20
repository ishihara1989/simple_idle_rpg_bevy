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
pub struct DungeonButton;

#[derive(Component)]
pub struct DungeonButtonText;

#[derive(Component)]
pub struct AutoRetryButton;

#[derive(Component)]
pub struct AutoRetryButtonText;

#[derive(Component)]
pub struct TabButton {
    pub tab: GameTab,
}

#[derive(Component)]
pub struct TabContent {
    pub tab: GameTab,
}

// Game state - separated into focused resources following ECS principles

#[derive(Resource)]
pub struct CombatState {
    pub is_game_over: bool,
    pub in_dungeon: bool,
}

#[derive(Resource)]
pub struct GameProgress {
    pub current_enemy_number: u32,
    pub has_died_once: bool,
}

#[derive(Resource)]
pub struct UIState {
    pub current_tab: GameTab,
}

#[derive(Resource)]
pub struct AutomationConfig {
    pub auto_retry_unlocked: bool,
    pub auto_retry_enabled: bool,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameTab {
    Combat,
    Rebirth,
    Automation,
}