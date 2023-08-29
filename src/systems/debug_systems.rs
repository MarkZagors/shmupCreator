use crate::components::*;
use bevy::prelude::*;

pub fn debug_collisions(mut gizmos: Gizmos, query: Query<(&CircleColider, &Transform)>) {
    for (circle_collider, transform) in &query {
        gizmos.circle_2d(
            Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            circle_collider.radius,
            Color::RED,
        );
    }
}
