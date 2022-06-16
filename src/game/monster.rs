use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, thread_rng};

use crate::{AppState, GameTextures};
use crate::game::{Enemy, GameDirection, Monster, Player};
use crate::game::utils::*;

const CHANCE_OF_SPAWNING: f64 = 0.1;

pub fn insert_monster_at(
    commands: &mut Commands,
    player_textures: &Res<GameTextures>,
    x: f32,
    y: f32,
) {
    spawn_object(commands,
                 create_sprite_bundle(player_textures.bug.clone(),
                                      (0.9, 0.9),
                                      (x, y + 0.5, 10.0)),
                 None,
                 None,
                 Collider::round_cuboid(0.25, 0.25, 0.1),
                 Enemy,
                 Monster {
                     speed: 2.0,
                     facing_direction: GameDirection::Right,
                 },
    );
}

pub fn death_by_enemy(
    mut commands: Commands,
    mut players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut state: ResMut<State<AppState>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for player in players.iter_mut() {
                for enemy in enemies.iter() {
                    if (*h1 == player && *h2 == enemy) || (*h1 == enemy && *h2 == player) {
                        commands.entity(player).despawn_recursive();
                        state
                            .set(AppState::DeathMenu)
                            .expect("Couldn't switch state to InGame");
                    }
                }
            }
        }
    }
}

pub fn add_enemies(commands: &mut Commands, world: &[usize], player_texture: &Res<GameTextures>) {
    world.iter().enumerate().for_each(|(x, height)| {
        if should_add_enemy(x) {
            insert_monster_at(commands, player_texture, x as f32, (*height + 1) as f32);
        }
    });
}

fn should_add_enemy(x: usize) -> bool {
    if x <= 5 {
        return false;
    }
    thread_rng().gen_bool(CHANCE_OF_SPAWNING)
}
