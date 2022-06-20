use bevy::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng, SeedableRng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

const JUMP_IMPULSE: f32 = 15.0;

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
pub struct Bullet;

#[derive(Component)]
pub struct WeakBullet;

#[derive(Component)]
pub struct StrongBullet;

#[derive(Component)]
pub struct PlayersBullet;

#[derive(Component)]
pub struct EnemyBullet;

#[derive(Component)]
pub enum Weapon {
    WeakBullet,
    StrongBullet,
}

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

impl Default for Jumper {
    fn default() -> Self {
        Jumper {
            jump_impulse: JUMP_IMPULSE,
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
pub struct Powerup;

#[derive(Component)]
pub struct Coffee;

#[derive(Component)]
pub struct Rust;

#[derive(Component, Default)]
pub struct Bug;

#[derive(Component, Default)]
pub struct Valgrind;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub direction: GameDirection,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            speed: 2.0,
            direction: GameDirection::Right,
        }
    }
}

#[derive(Debug, Component, PartialEq, Eq, Clone)]
pub struct Random {
    pub generator: Pcg64,
    pub seed: String,
    pub can_change: bool,
}

impl Random {
    pub fn new() -> Self {
        Random {
            generator: Pcg64::from_entropy(),
            seed: String::new(),
            can_change: false,
        }
    }

    fn generate_random_seed() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }

    pub fn add_char(&mut self, c: char) {
        if self.can_change && self.seed.len() <= 15 {
            // TODO podmienic 15 na stala
            self.seed.push(c);
        }
    }

    pub fn delete_last(&mut self) {
        if self.can_change {
            self.seed.pop();
        }
    }

    pub fn new_random_seed(&mut self) {
        self.seed = Random::generate_random_seed();
    }

    pub fn make_generator_for_level(&mut self, level: usize) {
        let temp_rng: Pcg64 = Seeder::from(&self.seed).make_rng();

        let level_seed: String = temp_rng
            .sample_iter(&Alphanumeric)
            .take(5 * level)
            .map(char::from)
            .collect();

        self.generator = Seeder::from(level_seed).make_rng();
    }
}

pub struct Level {
    pub level: usize,
    pub difficulty: f64,
}

impl Level {
    pub fn new() -> Self {
        Self {
            level: 1,
            difficulty: 1.,
        }
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
        self.update_difficulty();
    }

    pub fn reset_level(&mut self) {
        self.level = 1;
        self.update_difficulty();
    }

    fn update_difficulty(&mut self) {
        self.difficulty = ((self.level - 1) / 3) as f64 + 1.;
    }
}

//TODO redundant
#[derive(Component, Default)]
pub struct LivingBeing;

#[derive(Component)]
pub struct PhantomEntity;
