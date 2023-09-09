mod components;
mod setup;
mod systems;

use systems::{
    bullet_systems, collision_systems, debug_systems, menu_systems, player_systems, resize_systems,
    spawner_systems,
};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use components::{AppState, PlayerAttackTimer, WindowSize};

const DEBUG: bool = true;

fn main() {
    let mut app = App::new();

    //STATES
    app.add_state::<AppState>();

    //PLUGINS
    app.add_plugins((
        DefaultPlugins,
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
    ));

    //RESOURCES
    app.insert_resource(PlayerAttackTimer(Timer::from_seconds(
        0.2,
        TimerMode::Repeating,
    )));
    app.insert_resource(WindowSize(Vec2 {
        x: 768.0,
        y: 1024.0,
    }));

    //SYSTEMS
    app.add_systems(
        OnEnter(AppState::InGame),
        (setup::setup_cameras, setup::setup),
    );

    app.add_systems(
        Update,
        (
            bevy::window::close_on_esc,
            (
                resize_systems::on_window_resize_ingame,
                player_systems::player_set_input,
                player_systems::player_movement,
                spawner_systems::spawn_player_bullets,
                collision_systems::check_boss_collisions,
                collision_systems::despawn_bullets,
                bullet_systems::bullet_movement,
                debug_systems::debug_collisions.run_if(debug_enabled),
                bullet_systems::bullet_start_velocity.before(bullet_systems::bullet_movement),
            )
                .run_if(in_state(AppState::InGame)),
            (menu_systems::test_system).run_if(in_state(AppState::MainMenu)),
        ),
    );

    // app.add_systems(Update, menu_systems::test_system);

    app.run();
}

fn debug_enabled() -> bool {
    return DEBUG;
}
