use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{AppState, GameTextures, Random};
use crate::game::{Enemy, Bug, Player, SAFE_ZONE_WIDTH, LivingBeing, DeadPlayerEvent, Jumper};
use crate::game::living_being::LivingBeingDeathEvent;
use crate::game::utils::*;

const SPAWNING_PROBABILITY: f64 = 0.1;

fn spawn_enemy<T>(commands: &mut Commands, texture: Handle<Image>, enemy_type: T, x: f32, y: f32)
    where T: Component {
    spawn_object(commands,
                 create_sprite_bundle(texture, (0.9, 0.9), (x, y, 10.0)),
                 None,
                 None,
                 Collider::round_cuboid(0.25, 0.25, 0.1),
                 None,
                 Enemy,
                 enemy_type,
    );
}

fn spawn_bug(commands: &mut Commands, game_textures: &Res<GameTextures>, x: f32, y: f32) {
    spawn_enemy(commands, game_textures.bug.clone(), Bug::default(), x, y);
}

pub fn death_by_enemy(
    mut players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_dead_event: EventWriter<LivingBeingDeathEvent>,
    mut send_dead_player_event: EventWriter<DeadPlayerEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            for player in players.iter_mut() {
                for enemy in enemies.iter() {
                    if (*ent1 == player && *ent2 == enemy) || (*ent1 == enemy && *ent2 == player) {
                        send_dead_event.send(LivingBeingDeathEvent { entity: player });
                        send_dead_player_event.send(DeadPlayerEvent { entity: player });
                    }
                }
            }
        }
    }
}

pub fn add_enemies(commands: &mut Commands, world: &[(i32, usize)], game_textures: &Res<GameTextures>, rng: &mut ResMut<Random>) {
    world.iter().for_each(|&(x, height)| {
        if should_add_enemy(x, rng) {
            spawn_bug(commands, game_textures, x as f32, height as f32 + 1.5);
        }
    });
}

fn should_add_enemy(x: i32, rng: &mut ResMut<Random>) -> bool {
    if x <= SAFE_ZONE_WIDTH as i32 {
        return false;
    }
    rng.generator.gen_bool(SPAWNING_PROBABILITY)
}
