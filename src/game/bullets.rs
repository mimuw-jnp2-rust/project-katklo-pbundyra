use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::utils::*;
use super::GameDirection;
use crate::game::living_being::LivingBeingHitEvent;
use crate::game::{
    spawn_dynamic_object, Bullet, Enemy, EnemyBullet, Player, PlayersBullet, Wall, Weapon,
};
use crate::GameTextures;

pub struct ShootEvent;
pub struct FastShootEvent;

const WEAK_BULLET_SPEED: f32 = 8.25;
const STRONG_BULLET_SPEED: f32 = 18.5;
const ENEMY_BULLET_SPEED: f32 = 8.25;

#[derive(Copy, Clone)]
pub struct BulletOptions {
    pub x: f32,
    pub y: f32,
    pub direction: GameDirection,
    pub player_vex: f32,
}

fn spawn_bullet(
    commands: &mut Commands,
    texture: Handle<Image>,
    options: BulletOptions,
    def_vel: f32,
) -> Entity {
    let (vel_x, spawn_x) = match options.direction {
        GameDirection::Left => (-def_vel, -0.75),
        GameDirection::Right => (def_vel, 0.75),
    };
    let mut bullet_entity = spawn_dynamic_object(
        commands,
        create_sprite_bundle(texture, (0.5, 0.2), (options.x + spawn_x, options.y, 0.0)),
        Some(vel_x),
        Some(0.0),
    );
    bullet_entity = spawn_sensor_collider(
        commands,
        bullet_entity,
        Collider::round_cuboid(0.0, 0.0, 0.0),
    );
    commands.entity(bullet_entity).insert(Bullet).id()
}

pub fn spawn_strong_bullet(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    options: BulletOptions,
) {
    let bullet = spawn_bullet(
        commands,
        game_textures.strong_bullet.clone(),
        options,
        STRONG_BULLET_SPEED,
    );
    commands.entity(bullet).insert(PlayersBullet);
}

pub fn spawn_weak_bullet(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    options: BulletOptions,
) {
    let bullet = spawn_bullet(
        commands,
        game_textures.weak_bullet.clone(),
        options,
        WEAK_BULLET_SPEED,
    );
    commands.entity(bullet).insert(PlayersBullet);
}

pub fn spawn_enemy_bullet(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    options: BulletOptions,
) {
    let bullet = spawn_bullet(
        commands,
        game_textures.enemy_bullet.clone(),
        options,
        ENEMY_BULLET_SPEED,
    );
    commands.entity(bullet).insert(EnemyBullet);
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    walls: Query<Entity, With<Wall>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            match (
                bullets.get(*ent1),
                walls.get(*ent2),
                bullets.get(*ent2),
                walls.get(*ent1),
            ) {
                (Ok(bullet), Ok(_), _, _) | (_, _, Ok(bullet), Ok(_)) => {
                    commands.entity(bullet).despawn_recursive()
                }
                _ => {}
            }
        }
    }
}

pub fn kill_enemy(
    mut commands: Commands,
    bullets: Query<Entity, With<PlayersBullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for collision_event in collision_event.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            match (
                bullets.get(*ent1),
                enemies.get(*ent2),
                bullets.get(*ent2),
                enemies.get(*ent1),
            ) {
                (Ok(bullet), Ok(enemy), _, _) | (_, _, Ok(bullet), Ok(enemy)) => {
                    commands.entity(bullet).despawn_recursive();
                    commands.entity(enemy).despawn_recursive();
                }
                _ => {}
            }
        }
    }
}
