use bevy::{
    prelude::*,
    render::camera::{ScalingMode, Viewport},
};

use crate::components::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(768, 1024),
                    ..default()
                }),
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: 768.0,
                    height: 1024.0,
                },
                near: -10000.0,
                ..default()
            },
            ..default()
        },
        GameCamera,
    ));
}

pub fn setup(mut commands: Commands) {
    //PLAYER
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.25, 0.25),
                custom_size: Some(Vec2::new(5000.0, 5000.0)),
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
