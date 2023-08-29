mod rendering;

use bevy::prelude::*;
use rendering::renderer;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, renderer::setup);

    app.add_systems(Update, bevy::window::close_on_esc);

    app.run();
}
