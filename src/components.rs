use bevy::prelude::*;

//MARKERS
#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletPlayer;

#[derive(Component)]
pub struct BulletEnemy;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct Spawner;

#[derive(Component)]
pub struct PlayerBulletSpawer;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct UIGameContainer(pub Vec2);

//COMPONENTS
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
    pub angle: f32,
}

#[derive(Component)]
pub struct PlayerInput {
    pub movement_axis: Vec2,
    pub shift_pressed: bool,
}

#[derive(Component)]
pub struct CircleColider {
    pub radius: f32,
}

//RESOURCES
#[derive(Resource)]
pub struct PlayerAttackTimer(pub Timer);

#[derive(Resource)]
pub struct WindowSize(pub Vec2);
