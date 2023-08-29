use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Moving {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct PlayerInput {
    pub movement_axis: Vec2,
}
