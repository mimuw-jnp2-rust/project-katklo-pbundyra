use crate::game::{Booster, Bullet, Enemy, Player, StrongBullet};
use crate::GameTextures;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{GameDirection, WeakBullet};

const WEAK_BULLET_SPEED: f32 = 8.25;
const STRONG_BULLET_SPEED: f32 = 18.5;

#[derive(Copy, Clone)]
pub struct BulletOptions {
    pub x: f32,
    pub y: f32,
    pub direction: GameDirection,
    pub player_vex: f32,
}

pub fn insert_weak_bullet_at(
    commands: &mut Commands,
    options: BulletOptions,
    game_textures: &mut Res<GameTextures>,
) {
    let vel_x: f32;
    let spawn_x: f32;
    match options.direction {
        GameDirection::Left => {
            vel_x = -WEAK_BULLET_SPEED + options.player_vex * 0.15;
            spawn_x = -0.75;
        }
        GameDirection::Right => {
            vel_x = WEAK_BULLET_SPEED + options.player_vex * 0.15;
            spawn_x = 0.75
        }
    }
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.weak_laser.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 0.2)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Transform::from_xyz(options.x + spawn_x, options.y, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Sleeping::disabled())
        .insert(GravityScale(0.0))
        .insert(Velocity {
            linvel: Vec2::new(vel_x, 0.0),
            ..default()
        })
        .insert(Ccd::enabled())
        .insert(Collider::round_cuboid(0.25, 0.05, 0.1))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(WeakBullet)
        .insert(Bullet);
}

pub fn insert_strong_bullet_at(
    commands: &mut Commands,
    options: BulletOptions,
    game_textures: &mut Res<GameTextures>,
) {
    let vel_x: f32;
    let spawn_x: f32;
    match options.direction {
        GameDirection::Left => {
            vel_x = -STRONG_BULLET_SPEED + options.player_vex * 0.15;
            spawn_x = -0.75;
        }
        GameDirection::Right => {
            vel_x = STRONG_BULLET_SPEED + options.player_vex * 0.15;
            spawn_x = 0.75
        }
    }
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.strong_laser.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 0.2)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Transform::from_xyz(options.x + spawn_x, options.y, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Sleeping::disabled())
        .insert(GravityScale(0.0))
        .insert(Velocity {
            linvel: Vec2::new(vel_x, 0.0),
            ..default()
        })
        .insert(Ccd::enabled())
        .insert(Collider::round_cuboid(0.25, 0.05, 0.1))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(StrongBullet)
        .insert(Bullet);
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
    boosters: Query<Entity, With<Booster>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for bullet in bullets.iter() {
                for booster in boosters.iter() {
                    for player in players.iter() {
                        if (*h1 == bullet && *h2 != booster && *h2 != player && *h2 != bullet)
                            || (*h2 == bullet && *h1 != booster && *h1 != player && *h1 != bullet)
                        {
                            commands.entity(bullet).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

pub fn killing_enemies(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for collision_event in collision_event.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for bullet in bullets.iter() {
                for enemy in enemies.iter() {
                    if (*h1 == bullet && *h2 == enemy) || (*h1 == enemy && *h2 == bullet) {
                        commands.entity(enemy).despawn_recursive();
                    }
                }
            }
        }
    }
}
