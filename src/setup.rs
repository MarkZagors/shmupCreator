use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
};

use crate::components::*;

pub fn setup_cameras(mut commands: Commands) {
    //GAME VIEWPORT
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // no "background color", we need to see the main camera's output
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 1,
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
        RenderLayers::from_layers(&[0]),
        GameCamera,
    ));

    //IN GAME UI VIEW
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[1]),
    ));
}

pub fn setup(mut commands: Commands) {
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

    //GAME VIEWPORT BACKGROUND
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(10000.0, 10000.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1000.0),
            ..default()
        },
        ..default()
    },));

    //GAME UI CONTAINER
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.00, 0.5),
                custom_size: Some(Vec2::new(1000.0, 1000.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        RenderLayers::layer(1),
        UIGameContainer(Vec2::new(500.0, 500.0)),
    ));
}
