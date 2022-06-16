use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, thread_rng};

use crate::game::{Booster, Coffee, Player, Rust};
use crate::game::utils::create_sprite_bundle;
use crate::GameTextures;

use super::utils::*;

const CHANCE_OF_SPAWNING_COFFEE: f64 = 0.1;
const CHANCE_OF_SPAWNING_RUST: f64 = 0.03;

pub fn insert_coffee_at(
    commands: &mut Commands,
    player_textures: &Res<GameTextures>,
    x: f32,
    y: f32,
) {
    spawn_object(commands,
                 create_sprite_bundle(player_textures.coffee.clone(),
                                      (0.99, 0.99),
                                      (x, y, 10.0)),
                 None,
                 None,
                 Collider::round_cuboid(0.05, 0.05, 0.1),
                 Booster,
                 Coffee,
    );
}

pub fn insert_rust_at(
    commands: &mut Commands,
    player_textures: &Res<GameTextures>,
    x: f32,
    y: f32,
) {
    spawn_object(commands,
                 create_sprite_bundle(player_textures.rust.clone(),
                                      (0.99, 0.99),
                                      (x, y, 10.0)),
                 None,
                 None,
                 Collider::round_cuboid(0.05, 0.05, 0.1),
                 Booster,
                 Rust,
    );
}

pub fn drink_coffee(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player)>,
    coffees: Query<Entity, With<Coffee>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for (player_entity, mut player) in players.iter_mut() {
                for coffee in coffees.iter() {
                    if (*h1 == player_entity && *h2 == coffee)
                        || (*h1 == coffee && *h2 == player_entity)
                    {
                        player.increase_speed();
                        commands.entity(coffee).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn learn_rust(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player)>,
    rusts: Query<Entity, With<Rust>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for (player_entity, mut player) in players.iter_mut() {
                for rust in rusts.iter() {
                    if (*h1 == player_entity && *h2 == rust)
                        || (*h1 == rust && *h2 == player_entity)
                    {
                        player.powerup_weapon();
                        commands.entity(rust).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn add_boosters(commands: &mut Commands, world: &[usize], player_texture: Res<GameTextures>) {
    world.iter().enumerate().for_each(|(x, height)| {
        if should_add_coffee(x) {
            insert_coffee_at(commands, &player_texture, x as f32, *height as f32 + 0.25);
        } else if should_add_rust(x) {
            insert_rust_at(commands, &player_texture, x as f32, *height as f32 + 0.25);
        }
    });
}

fn should_add_coffee(x: usize) -> bool {
    if x <= 5 {
        return false;
    }
    thread_rng().gen_bool(CHANCE_OF_SPAWNING_COFFEE)
}

fn should_add_rust(x: usize) -> bool {
    if x <= 15 {
        return false;
    }
    thread_rng().gen_bool(CHANCE_OF_SPAWNING_RUST)
}
