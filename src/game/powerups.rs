use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{GameTextures, Random};
use crate::game::{Coffee, Player, Powerup, Rust};

use super::utils::*;

pub struct CoffeeEvent;

pub struct RustEvent;

const SPAWNING_COFFEE_PROBABILITY: f64 = 0.1;
const SPAWNING_RUST_PROBABILITY: f64 = 0.03;
pub const COFFEE_DURATION: u64 = 10;
pub const RUST_DURATION: u64 = 7;
const SAFE_ZONE_WIDTH: i32 = 5;

fn spawn_powerup<T>(commands: &mut Commands, texture: Handle<Image>, powerup_type: T, x: f32, y: f32) where T: Component {
    let mut powerup_entity = spawn_static_object(commands, create_sprite_bundle(texture, (0.99, 0.99), (x, y, 10.0)));
    powerup_entity = spawn_sensor_collider(commands, powerup_entity, Collider::round_cuboid(0.4, 0.4, 0.1));

    commands.entity(powerup_entity)
        .insert(Powerup)
        .insert(powerup_type);
}

fn spawn_coffee(commands: &mut Commands, game_textures: &Res<GameTextures>, x: f32, y: f32) {
    spawn_powerup(commands, game_textures.coffee.clone(), Coffee, x, y);
}

fn spawn_rust(commands: &mut Commands, game_textures: &Res<GameTextures>, x: f32, y: f32) {
    spawn_powerup(commands, game_textures.rust.clone(), Rust, x, y);
}

pub fn drink_coffee(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player)>,
    coffees: Query<Entity, With<Coffee>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_event: EventWriter<CoffeeEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            if let Ok((player_entity, mut player)) = players.get_single_mut() {
                for coffee in coffees.iter() {
                    if (*h1 == player_entity && *h2 == coffee)
                        || (*h1 == coffee && *h2 == player_entity) {
                        player.increase_speed();
                        commands.entity(coffee).despawn_recursive();
                        send_event.send(CoffeeEvent);
                    }
                }
            }
        }
    }
}

pub fn finish_coffee(mut players: Query<&mut Player>, time: Res<Time>) {
    if let Ok(mut player) = players.get_single_mut() {
        player.coffee_timer.tick(time.delta());
        if player.coffee_timer.finished() {
            player.decrease_speed();
        }
    }
}


pub fn learn_rust(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player)>,
    rusts: Query<Entity, With<Rust>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_event: EventWriter<RustEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            if let Ok((player_entity, mut player)) = players.get_single_mut() {
                for rust in rusts.iter() {
                    if (*h1 == player_entity && *h2 == rust)
                        || (*h1 == rust && *h2 == player_entity)
                    {
                        send_event.send(RustEvent);
                        player.powerup_weapon();
                        commands.entity(rust).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn degrade_weapon(mut players: Query<&mut Player>, time: Res<Time>) {
    if let Ok(mut player) = players.get_single_mut() {
        player.weapon_upgrade_timer.tick(time.delta());
        if player.weapon_upgrade_timer.finished() {
            player.degrade_weapon();
        }
    }
}


pub fn add_powerups(commands: &mut Commands, world: &[(i32, usize)], game_textures: Res<GameTextures>, rng: &mut ResMut<Random>) {
    world.iter().for_each(|&(x, height)| {
        if should_add_coffee(x, rng) {
            spawn_coffee(commands, &game_textures, x as f32, height as f32 + 0.75);
        }
        if should_add_rust(x, rng) {
            spawn_rust(commands, &game_textures, x as f32, height as f32 + 0.75);
        }
    });
}

fn should_add_coffee(x: i32, rng: &mut ResMut<Random>) -> bool {
    if x <= SAFE_ZONE_WIDTH {
        return false;
    }
    rng.generator.gen_bool(SPAWNING_COFFEE_PROBABILITY)
}

fn should_add_rust(x: i32, rng: &mut ResMut<Random>) -> bool {
    if x <= SAFE_ZONE_WIDTH {
        return false;
    }
    rng.generator.gen_bool(SPAWNING_RUST_PROBABILITY)
}
