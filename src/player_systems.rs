use bevy::prelude::*;

use crate::components::{Moving, Player, PlayerInput};

const PLAYER_SPEED_NORMAL: f32 = 400.0;
const PLAYER_SPEED_FOCUSED: f32 = 150.0;

pub fn player_set_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerInput, With<Player>>,
) {
    let mut player_input = query.single_mut();

    player_input.movement_axis.x = 0.0;
    player_input.movement_axis.y = 0.0;

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Left) {
        player_input.movement_axis.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Right) {
        player_input.movement_axis.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        player_input.movement_axis.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        player_input.movement_axis.y -= 1.0;
    }

    player_input.movement_axis = player_input.movement_axis.normalize_or_zero();

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        player_input.shift_pressed = true;
    } else {
        player_input.shift_pressed = false;
    }
}

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Moving, &mut Transform, &PlayerInput), With<Player>>,
) {
    let (mut player_movement, mut player_transform, player_input) = query.single_mut();

    let movement_speed: f32 = if player_input.shift_pressed {
        PLAYER_SPEED_FOCUSED
    } else {
        PLAYER_SPEED_NORMAL
    };

    player_movement.velocity.x = player_input.movement_axis.x * movement_speed;
    player_movement.velocity.y = player_input.movement_axis.y * movement_speed;

    player_transform.translation.x += player_movement.velocity.x * time.delta_seconds();
    player_transform.translation.y += player_movement.velocity.y * time.delta_seconds();
}
