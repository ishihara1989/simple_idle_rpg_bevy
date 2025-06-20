use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod events;
pub mod ui;
pub mod plugins;

#[cfg(test)]
pub mod tests {
    pub mod components_tests;
    pub mod systems_tests;
    pub mod integration_tests;
    pub mod real_time_combat_tests;
}

#[derive(Resource)]
pub struct StartupConfig {
    pub level: u32,
    pub experience: u64,
    pub hp_level: u32,
    pub attack_level: u32,
    pub defense_level: u32,
    pub speed_level: u32,
    pub duration: u64,
}

pub use components::*;
pub use systems::*;
pub use events::*;
pub use ui::*;
pub use plugins::*;