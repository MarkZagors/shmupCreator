use crate::components::Moving;
use bevy::prelude::*;

pub fn bullet_movement(time: Res<Time>, mut query: Query<(&mut Moving, &mut Transform)>) {
    for (moving, mut transform) in &mut query {
        transform.translation.x += moving.velocity.x * time.delta_seconds();
        transform.translation.y += moving.velocity.y * time.delta_seconds();
    }
}
