use crate::components::*;
use bevy::prelude::*;

pub fn check_boss_collisions(
    boss_query: Query<(&CircleColider, &Transform), With<Boss>>,
    bullet_query: Query<(&CircleColider, &Transform), (With<Bullet>, With<BulletPlayer>)>,
) {
    let (boss_collision, boss_transform) = &boss_query.get_single().unwrap();

    for (bullet_collision, bullet_transform) in &bullet_query {
        let distance_sq = distance_squared(boss_transform, bullet_transform);
        let radius_sum = boss_collision.radius + bullet_collision.radius;
        if distance_sq < radius_sum * radius_sum {
            println!("Collision!");
        }
    }
}

fn distance_squared(pos1: &Transform, pos2: &Transform) -> f32 {
    let x_distance = pos2.translation.x - pos1.translation.x;
    let y_distance = pos2.translation.y - pos1.translation.y;
    return x_distance * x_distance + y_distance * y_distance;
}
