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

pub use components::*;
pub use systems::*;
pub use events::*;
pub use ui::*;
pub use plugins::*;