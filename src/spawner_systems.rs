use crate::components::*;
use bevy::prelude::*;

pub fn spawn_player_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PlayerAttackTimer>,
    query: Query<&Transform, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let player_transform = query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.25),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(player_transform.translation),
                ..default()
            },
            Moving {
                velocity: Vec2 { x: 0.0, y: 200.0 },
            },
            Bullet,
        ));
    }
}
