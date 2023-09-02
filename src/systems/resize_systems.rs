use crate::components::*;
use bevy::{prelude::*, window::WindowResized};

pub fn on_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut game_camera_query: Query<&mut Camera, With<GameCamera>>,
    mut ui_container_query: Query<(&mut Sprite, &UIGameContainer), With<UIGameContainer>>,
    game_window_size: Res<WindowSize>,
) {
    for app_window in resize_events.iter() {
        let mut camera = game_camera_query.single_mut();
        let (mut ui_container_sprite, ui_container_window) = ui_container_query.single_mut();

        let app_window_vec = Vec2::new(app_window.width, app_window.height);

        match &mut camera.viewport {
            Some(viewport) => {
                let mut viewport_window_pos = Vec2::default();
                let mut viewport_window_size = Vec2::default();

                keep_aspect_viewport(
                    app_window_vec,
                    &game_window_size,
                    &mut viewport_window_size,
                    &mut viewport_window_pos,
                );

                viewport.physical_position =
                    UVec2::new(viewport_window_pos.x as u32, viewport_window_pos.y as u32);
                viewport.physical_size =
                    UVec2::new(viewport_window_size.x as u32, viewport_window_size.y as u32);
            }
            None => panic!("Viewport failed to be found."),
        };

        match &mut ui_container_sprite.custom_size {
            Some(container_sprite_size) => {
                let ui_container_vec: Vec2 = Vec2 {
                    x: ui_container_window.0.x,
                    y: ui_container_window.0.y,
                };
                keep_aspect_center(app_window_vec, ui_container_vec, container_sprite_size);
                println!("{:?}", container_sprite_size);
            }
            None => panic!("container_sprite failed to be found."),
        };
    }
}

fn keep_aspect_viewport(
    app_window_vec: Vec2,
    game_window_size: &Res<'_, WindowSize>,
    viewport_window_size: &mut Vec2,
    viewport_window_pos: &mut Vec2,
) {
    let window_center = Vec2::new(app_window_vec.x / 2.0, app_window_vec.y / 2.0);

    let ratio_xy = game_window_size.0.x / game_window_size.0.y;
    let ratio_yx = game_window_size.0.y / game_window_size.0.x;
    let resized_window_x = app_window_vec.y * ratio_xy;

    if resized_window_x > app_window_vec.x {
        viewport_window_size.x = app_window_vec.x;
        viewport_window_size.y = app_window_vec.x * ratio_yx;

        let center_y = (window_center.y - viewport_window_size.y / 2.0).max(0.0);
        viewport_window_pos.y = center_y;
        viewport_window_pos.x = 0.0;
    } else {
        viewport_window_size.y = app_window_vec.y;
        viewport_window_size.x = app_window_vec.y * ratio_xy;

        let center_x = (window_center.x - viewport_window_size.x / 2.0).max(0.0);
        viewport_window_pos.x = center_x;
        viewport_window_pos.y = 0.0;
    }
}

fn keep_aspect_center(app_window_vec: Vec2, ui_window_size: Vec2, viewport_window_size: &mut Vec2) {
    let ratio_xy = ui_window_size.x / ui_window_size.y;
    let ratio_yx = ui_window_size.y / ui_window_size.x;
    let resized_window_x = app_window_vec.y * ratio_xy;

    if resized_window_x > app_window_vec.x {
        viewport_window_size.x = app_window_vec.x;
        viewport_window_size.y = app_window_vec.x * ratio_yx;
    } else {
        viewport_window_size.y = app_window_vec.y;
        viewport_window_size.x = app_window_vec.y * ratio_xy;
    }
}
