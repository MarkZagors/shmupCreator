use bevy::prelude::*;

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    //PLAYER
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.25, 0.25),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50.0, -50.0, 0.0)),
            ..default()
        },
        VelocityComponent {
            velocity: Vec2 { x: 100.0, y: 100.0 },
        },
        PlayerInput {
            movement_axis: Vec2 { x: 0.0, y: 0.0 },
            shift_pressed: false,
        },
        Player,
    ));

    //PLAYER BULLET SPAWNER
    commands.spawn((TransformBundle::default(), Spawner, PlayerBulletSpawer));


}
