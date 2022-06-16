use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum GameDirection {
    Left,
    Right,
}


#[derive(Component)]
pub struct FinishLine;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct WeakBullet;

#[derive(Component)]
pub struct StrongBullet;

#[derive(Component)]
pub enum Weapon {
    WeakBullet,
    StrongBullet,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Booster;

#[derive(Component)]
pub struct Coffee;

#[derive(Component)]
pub struct Rust;

#[derive(Component)]
pub struct Monster {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

#[derive(Component)]
pub struct Enemy;
