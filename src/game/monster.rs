use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::utils::*;
use crate::game::{
    Bug, DeadPlayerEvent, Enemy, EnemyBullet, Jumper, Player, Valgrind, SAFE_ZONE_WIDTH,
};
use crate::{GameTextures, Level, Random};

const SPAWNING_PROBABILITY: f64 = 0.05;

fn spawn_enemy<T>(commands: &mut Commands, texture: Handle<Image>, enemy_type: T, x: f32, y: f32)
where
    T: Component,
{
    let mut enemy_entity = spawn_dynamic_object(
        commands,
        create_sprite_bundle(texture, Vec2::new(0.9, 0.9), Vec3::new(x, y, 10.0)),
        None,
        None,
    );
    enemy_entity = spawn_solid_collider(
        commands,
        enemy_entity,
        Collider::round_cuboid(0.25, 0.25, 0.1),
        None,
    );
    commands
        .entity(enemy_entity)
        .insert(Enemy::default())
        .insert(Jumper::default())
        .insert(enemy_type);
}

fn spawn_bug(commands: &mut Commands, game_textures: &Res<GameTextures>, x: f32, y: f32) {
    spawn_enemy(commands, game_textures.bug.clone(), Bug::default(), x, y);
}

fn spawn_valgrind(commands: &mut Commands, game_textures: &Res<GameTextures>, x: f32, y: f32) {
    spawn_enemy(
        commands,
        game_textures.valgrind.clone(),
        Valgrind::default(),
        x,
        y,
    );
}

pub fn death_by_enemy(
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    enemy_bullets: Query<Entity, With<EnemyBullet>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_dead_player_event: EventWriter<DeadPlayerEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            let death_reason_from_collision = players
                .get(*ent1)
                .and(enemy_bullets.get(*ent2).or_else(|_| enemies.get(*ent2)))
                .or_else(|_| {
                    players
                        .get(*ent2)
                        .and(enemy_bullets.get(*ent1).or_else(|_| enemies.get(*ent1)))
                });

            if death_reason_from_collision.is_ok() {
                send_dead_player_event.send(DeadPlayerEvent)
            }
        }
    }
}

pub fn add_enemies(
    commands: &mut Commands,
    world: &[(i32, usize)],
    game_textures: &Res<GameTextures>,
    rng: &mut ResMut<Random>,
    level: &Res<Level>,
) {
    // every 3 levels we change the kinds of enemies.
    let (should_add_bug, should_add_valgrind) = (level.level % 3 != 2, level.level % 3 != 1);

    world.iter().for_each(|&(x, height)| {
        if should_add_bug && should_add_enemy(x, rng, level) {
            spawn_bug(commands, game_textures, x as f32, height as f32 + 1.5);
        }
        if should_add_valgrind && should_add_enemy(x, rng, level) {
            spawn_valgrind(commands, game_textures, x as f32, height as f32 + 1.5);
        }
    });
}

fn should_add_enemy(x: i32, rng: &mut ResMut<Random>, level: &Res<Level>) -> bool {
    if x <= SAFE_ZONE_WIDTH as i32 {
        return false;
    }
    rng.generator
        .gen_bool(SPAWNING_PROBABILITY * level.difficulty)
}
