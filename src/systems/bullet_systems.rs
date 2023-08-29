use crate::components::*;
use bevy::prelude::*;

pub fn bullet_start_velocity(mut query: Query<(&mut Velocity, &Speed), With<Bullet>>) {
    for (mut bullet_velocity, bullet_speed) in &mut query {
        bullet_velocity.0.x = bullet_speed.angle.to_radians().cos() * bullet_speed.speed;
        bullet_velocity.0.y = bullet_speed.angle.to_radians().sin() * bullet_speed.speed;
    }
}

pub fn bullet_movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Bullet>>,
) {
    for (bullet_velocity, mut transform) in &mut query {
        transform.translation.x += bullet_velocity.0.x * time.delta_seconds();
        transform.translation.y += bullet_velocity.0.y * time.delta_seconds();
    }
}
