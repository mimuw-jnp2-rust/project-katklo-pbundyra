use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::{spawn_enemy_bullet, BulletOptions, Enemy, Powerup, Valgrind};
use crate::{GameTextures, Random};

use super::super::AppState;
use super::{GameDirection, Jumper};

const JUMP_PROBABILITY: f64 = 0.25;
const SHOOT_PROBABILITY: f64 = 0.25;
const CHANGE_DIRECTION_PROBABILITY: f64 = 0.25;
const ACTION_TIMESTEP: f64 = 2.0;

pub struct MonsterAiPlugin;
struct MonsterCollisionEvent {
    pub entity: Entity,
}

impl Plugin for MonsterAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(monster_walks)
                .with_system(monster_contact_detection)
                .with_system(monster_change_direction_on_contact),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(ACTION_TIMESTEP))
                .with_system(monster_changes_direction_randomly)
                .with_system(monster_jumps)
                .with_system(valgrind_shoots),
        )
        .add_event::<MonsterCollisionEvent>();
    }
}

fn monster_walks(mut monsters: Query<(&Enemy, &mut Velocity)>) {
    for (monster, mut velocity) in monsters.iter_mut() {
        let speed = match monster.direction {
            GameDirection::Left => -monster.speed,
            GameDirection::Right => monster.speed,
        };

        velocity.linvel = Vec2::new(speed, velocity.linvel.y);
    }
}

fn valgrind_shoots(
    mut commands: Commands,
    mut game_textures: Res<GameTextures>,
    positions: Query<(&mut Transform, &RigidBody, &mut Enemy, &mut Velocity), With<Valgrind>>,
    mut rng: ResMut<Random>,
) {
    for (pos, _, valgrind, vel) in positions.iter() {
        let options = BulletOptions {
            x: pos.translation.x,
            y: pos.translation.y,
            direction: valgrind.direction,
            player_vex: vel.linvel.x,
        };
        if should_shoot(&mut rng) {
            spawn_enemy_bullet(&mut commands, &mut game_textures, options);
        }
    }
}

fn change_direction(mut monster: Mut<Enemy>) {
    monster.direction = match monster.direction {
        GameDirection::Left => GameDirection::Right,
        GameDirection::Right => GameDirection::Left,
    }
}

fn monster_changes_direction_randomly(
    mut monster_query: Query<&mut Enemy>,
    mut rng: ResMut<Random>,
) {
    for monster in monster_query.iter_mut() {
        if should_change_direction(&mut rng) {
            change_direction(monster);
        }
    }
}

fn monster_contact_detection(
    monsters: Query<Entity, With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_monster_collision: EventWriter<MonsterCollisionEvent>,
    powerups: Query<Entity, With<Powerup>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            match (monsters.get(*ent1), monsters.get(*ent2)) {
                (Ok(monster), _) | (_, Ok(monster)) => {
                    match (powerups.get(*ent1), powerups.get(*ent2)) {
                        (Ok(_), _) | (_, Ok(_)) => {}
                        _ => send_monster_collision.send(MonsterCollisionEvent { entity: monster }),
                    }
                }
                _ => {}
            }
        }
    }
}

fn monster_change_direction_on_contact(
    mut events: EventReader<MonsterCollisionEvent>,
    mut monster_query: Query<&mut Enemy>,
) {
    for event in events.iter() {
        if let Ok(monster) = monster_query.get_mut(event.entity) {
            change_direction(monster);
        }
    }
}

fn monster_jumps(
    mut monsters: Query<(&mut Jumper, &mut Velocity), With<Enemy>>,
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

fn should_shoot(rng: &mut ResMut<Random>) -> bool {
    rng.generator.gen_bool(SHOOT_PROBABILITY)
}
