mod bullet_systems;
mod components;
mod renderer;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, renderer::setup);

    app.add_systems(Update, bevy::window::close_on_esc);
    app.add_systems(Update, bullet_systems::bullet_movement);

    app.run();
}
