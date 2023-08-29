use bevy::prelude::*;

//MARKERS
#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spawner;

#[derive(Component)]
pub struct PlayerBulletSpawer;

//COMPONENTS
#[derive(Component)]
pub struct Moving {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct PlayerInput {
    pub movement_axis: Vec2,
    pub shift_pressed: bool,
}

//RESOURCES
#[derive(Resource)]
pub struct PlayerAttackTimer(pub Timer);
