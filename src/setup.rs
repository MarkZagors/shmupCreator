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
        Velocity(Vec2::default()),
        PlayerInput {
            movement_axis: Vec2 { x: 0.0, y: 0.0 },
            shift_pressed: false,
        },
        Player,
    ));

    //PLAYER BULLET SPAWNER
    commands.spawn((TransformBundle::default(), Spawner, PlayerBulletSpawer));

    //BOSS
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 1.00, 0.25),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50.0, 50.0, 0.0)),
            ..default()
        },
        CircleColider { radius: 30.0 },
        Boss,
    ));
}
