use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::{AudioEvent, AudioType, Coffee, Player, Powerup, Rust};
use crate::{GameTextures, Level, Random};

use super::utils::*;

pub struct PowerupsPlugin;

pub struct CoffeeEvent {
    coffee: Entity,
}

pub struct RustEvent {
    rust: Entity,
}

const SPAWNING_COFFEE_PROBABILITY: f64 = 0.05;
const SPAWNING_RUST_PROBABILITY: f64 = 0.03;
pub const COFFEE_DURATION: u64 = 10;
pub const RUST_DURATION: u64 = 7;
const SAFE_ZONE_WIDTH: i32 = 5;

impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drink_coffee)
            .add_system(handle_coffee_event)
            .add_system(finish_coffee)
            .add_system(learn_rust)
            .add_system(handle_rust_event)
            .add_system(finish_rust)
            .add_event::<CoffeeEvent>()
            .add_event::<RustEvent>();
    }
}

fn spawn_powerup<T>(
    commands: &mut Commands,
    texture: Handle<Image>,
    powerup_type: T,
    x: f32,
    y: f32,
) where
    T: Component,
{
    let mut powerup_entity = spawn_static_object(
        commands,
        create_sprite_bundle(texture, Vec2::new(0.99, 0.99), Vec3::new(x, y, 10.0)),
    );
    powerup_entity = spawn_sensor_collider(
        commands,
        powerup_entity,
        Collider::round_cuboid(0.4, 0.4, 0.1),
    );
    commands
        .entity(powerup_entity)
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
    players: Query<(Entity, &mut Player)>,
    coffees: Query<Entity, With<Coffee>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_event: EventWriter<CoffeeEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            match (
                players.get(*ent1),
                coffees.get(*ent2),
                players.get(*ent2),
                coffees.get(*ent1),
            ) {
                (Ok(_), Ok(coffee), _, _) | (_, _, Ok(_), Ok(coffee)) => {
                    send_event.send(CoffeeEvent { coffee });
                }
                _ => {}
            }
        }
    }
}

fn handle_coffee_event(
    mut commands: Commands,
    mut coffee_events: EventReader<CoffeeEvent>,
    mut players: Query<&mut Player>,
    mut audio_event_sender: EventWriter<AudioEvent>,
) {
    if let Ok(mut player) = players.get_single_mut() {
        coffee_events.iter().for_each(|coffee_event| {
            player.increase_speed();
            commands.entity(coffee_event.coffee).despawn_recursive();
            audio_event_sender.send(AudioEvent::new(AudioType::Coffee));
        });
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
    players: Query<(Entity, &mut Player)>,
    rusts: Query<Entity, With<Rust>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_event: EventWriter<RustEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            match (
                players.get(*ent1),
                rusts.get(*ent2),
                players.get(*ent2),
                rusts.get(*ent1),
            ) {
                (Ok(_), Ok(rust), _, _) | (_, _, Ok(_), Ok(rust)) => {
                    send_event.send(RustEvent { rust });
                }
                _ => {}
            }
        }
    }
}

fn handle_rust_event(
    mut commands: Commands,
    mut rust_events: EventReader<RustEvent>,
    mut players: Query<&mut Player>,
    mut audio_event_sender: EventWriter<AudioEvent>,
) {
    if let Ok(mut player) = players.get_single_mut() {
        rust_events.iter().for_each(|rust_event| {
            player.upgrade_weapon();
            commands.entity(rust_event.rust).despawn_recursive();
            audio_event_sender.send(AudioEvent::new(AudioType::Rust));
        });
    }
}

pub fn finish_rust(mut players: Query<&mut Player>, time: Res<Time>) {
    if let Ok(mut player) = players.get_single_mut() {
        player.weapon_upgrade_timer.tick(time.delta());

        if player.weapon_upgrade_timer.finished() {
            player.degrade_weapon();
        }
    }
}

pub fn add_powerups(
    commands: &mut Commands,
    world: &[(i32, usize)],
    game_textures: Res<GameTextures>,
    rng: &mut ResMut<Random>,
    level: &Res<Level>,
) {
    world.iter().for_each(|&(x, height)| {
        let y = height as f32 + 0.75;

        if should_add(x, rng, level, SPAWNING_COFFEE_PROBABILITY) {
            spawn_coffee(commands, &game_textures, x as f32, y);
        }
        if should_add(x, rng, level, SPAWNING_RUST_PROBABILITY) {
            spawn_rust(commands, &game_textures, x as f32, y);
        }
    });
}

fn should_add(x: i32, rng: &mut ResMut<Random>, level: &Res<Level>, chance: f64) -> bool {
    if x <= SAFE_ZONE_WIDTH {
        return false;
    }
    rng.generator.gen_bool(chance / level.difficulty)
}
