use crate::components::*;
use bevy::{prelude::*, window::WindowResized};

pub fn on_window_resize_ingame(
    mut resize_events: EventReader<WindowResized>,
    mut game_camera_query: Query<&mut Camera, With<GameCamera>>,
    game_window_size: Res<WindowSize>,
) {
    for app_window in resize_events.iter() {
        let mut camera = game_camera_query.single_mut();

        let app_window_vec = Vec2::new(app_window.width, app_window.height);

        match &mut camera.viewport {
            Some(viewport) => {
                let mut viewport_window_pos = Vec2::default();
                let mut viewport_window_size = Vec2::default();
                let game_window_extended_vec =
                    Vec2::new(game_window_size.0.x, game_window_size.0.y);

                keep_aspect_viewport(
                    app_window_vec,
                    game_window_extended_vec,
                    &mut viewport_window_size,
                    &mut viewport_window_pos,
                    Vec2 { x: 20.0, y: 20.0 },
                );

                viewport.physical_position =
                    UVec2::new(viewport_window_pos.x as u32, viewport_window_pos.y as u32);
                viewport.physical_size =
                    UVec2::new(viewport_window_size.x as u32, viewport_window_size.y as u32);
            }
            None => panic!("Viewport failed to be found."),
        };
    }
}

fn keep_aspect_viewport(
    app_window_vec: Vec2,
    game_window_size: Vec2,
    viewport_window_size: &mut Vec2,
    viewport_window_pos: &mut Vec2,
    margin_vector: Vec2,
) {
    let window_center = Vec2::new(app_window_vec.x / 2.0, app_window_vec.y / 2.0);

    let ratio_xy = game_window_size.x / game_window_size.y;
    let ratio_yx = game_window_size.y / game_window_size.x;
    let resized_window_x = app_window_vec.y * ratio_xy;

    if resized_window_x > app_window_vec.x {
        let x_size = (app_window_vec.x - margin_vector.x * 2.0).max(10.0);

        viewport_window_size.x = x_size;
        viewport_window_size.y = x_size * ratio_yx;

        let center_y = (window_center.y - viewport_window_size.y / 2.0).max(0.0);
        viewport_window_pos.y = center_y;
        viewport_window_pos.x = margin_vector.x;
    } else {
        let y_size = (app_window_vec.y - margin_vector.y * 2.0).max(10.0);

        viewport_window_size.y = y_size;
        viewport_window_size.x = y_size * ratio_xy;

        let center_x = (window_center.x - viewport_window_size.x / 2.0).max(0.0);
        viewport_window_pos.x = center_x;
        viewport_window_pos.y = margin_vector.y;
    }
}
