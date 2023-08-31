mod components;
mod setup;
mod systems;

use systems::{bullet_systems, collision_systems, debug_systems, player_systems, spawner_systems};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use components::PlayerAttackTimer;

const DEBUG: bool = true;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.insert_resource(PlayerAttackTimer(Timer::from_seconds(
        0.2,
        TimerMode::Repeating,
    )));

    app.add_systems(Startup, setup::setup);

    app.add_systems(Update, bevy::window::close_on_esc);
    app.add_systems(Update, player_systems::player_set_input);
    app.add_systems(Update, player_systems::player_movement);
    app.add_systems(Update, spawner_systems::spawn_player_bullets);
    app.add_systems(Update, collision_systems::check_boss_collisions);

    app.add_systems(
        Update,
        bullet_systems::bullet_start_velocity.before(bullet_systems::bullet_movement),
    );
    app.add_systems(Update, bullet_systems::bullet_movement);

    if DEBUG {
        app.add_systems(Update, debug_systems::debug_collisions);
    }

    app.run();
}
