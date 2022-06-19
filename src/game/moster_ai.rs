use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::living_being::{LivingBeingDeathEvent, LivingBeingHitEvent};
use crate::Random;

use super::super::AppState;
use super::{Bug, GameDirection, Jumper};

const JUMP_PROBABILITY: f64 = 0.25;
const CHANGE_DIRECTION_PROBABILITY: f64 = 0.25;
const ACTION_TIMESTEP: f64 = 2.0;

pub struct MonsterAiPlugin;

impl Plugin for MonsterAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(monster_walks))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(ACTION_TIMESTEP))
                    .with_system(monster_changes_direction)
                    .with_system(monster_jumps),
            )
            .add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>();
    }
}

fn monster_walks(mut monsters: Query<(&Bug, &mut Velocity)>) {
    for (monster, mut velocity) in monsters.iter_mut() {
        let speed = match monster.facing_direction {
            GameDirection::Left => -monster.speed,
            GameDirection::Right => monster.speed,
        };

        velocity.linvel = Vec2::new(speed, velocity.linvel.y);
    }
}

fn monster_changes_direction(mut monster_query: Query<&mut Bug>, mut rng: ResMut<Random>) {
    for mut monster in monster_query.iter_mut() {
        if should_change_direction(&mut rng) {
            monster.facing_direction = match monster.facing_direction {
                GameDirection::Left => GameDirection::Right,
                GameDirection::Right => GameDirection::Left,
            }
        }
    }
}

fn monster_jumps(
    mut monsters: Query<(&mut Jumper, &mut Velocity), With<Bug>>,
    mut rng: ResMut<Random>,
) {
    for (mut jumper, mut velocity) in monsters.iter_mut() {
        if should_jump(&mut rng) {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse);
            jumper.is_jumping = true
        }
    }
}

fn should_change_direction(rng: &mut ResMut<Random>) -> bool {
    rng.generator.gen_bool(CHANGE_DIRECTION_PROBABILITY)
}

fn should_jump(rng: &mut ResMut<Random>) -> bool {
    rng.generator.gen_bool(JUMP_PROBABILITY)
}
