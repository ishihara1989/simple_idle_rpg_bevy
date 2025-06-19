use bevy::prelude::*;
use simple_idle_rpg::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            PlayerPlugin,
            CombatPlugin,
            StatsPlugin,
            UIPlugin,
        ))
        .run();
}

