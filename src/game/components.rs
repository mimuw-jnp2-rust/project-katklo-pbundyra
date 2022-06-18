use bevy::prelude::*;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

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

impl Default for Jumper {
    fn default() -> Self {
        Jumper {
            jump_impulse: 13.0,
            is_jumping: false,
        }
    }
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
pub struct Bug {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

impl Default for Bug {
    fn default() -> Self {
        Bug {
            speed: 2.0,
            facing_direction: GameDirection::Right,
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Debug, Component, PartialEq, Eq, Clone)]
pub struct Random {
    pub generator: Pcg64,
    pub seed: String,
}

impl Random {
    pub fn generate_random_seed() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    pub fn with_random_seed() -> Self {
        Random::from_seed(Random::generate_random_seed())
    }


    pub fn from_seed(seed: String) -> Self {
        Random {
            generator: Seeder::from(&seed).make_rng(),
            seed,
        }
    }
}
