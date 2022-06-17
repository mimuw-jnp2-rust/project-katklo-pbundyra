use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{Bullet, Enemy, Player, Weapon};
use crate::GameTextures;

use super::{GameDirection};
use super::utils::*;

const WEAK_BULLET_SPEED: f32 = 8.25;
const STRONG_BULLET_SPEED: f32 = 18.5;

#[derive(Copy, Clone)]
pub struct BulletOptions {
    pub x: f32,
    pub y: f32,
    pub direction: GameDirection,
    pub player_vex: f32,
}

fn spawn_bullet(commands: &mut Commands, texture: Handle<Image>, bullet_type: Weapon, options: BulletOptions, def_vel : f32) {
    let (vel_x, spawn_x) = match options.direction {
        GameDirection::Left => {
            (-def_vel + options.player_vex * 0.15, -0.75)
        }
        GameDirection::Right => {
            (def_vel + options.player_vex * 0.15,  0.75)
        }
    };

    spawn_object(commands,
                 create_sprite_bundle(texture, (0.5, 0.2), (options.x + spawn_x, options.y, 0.0)),
                 Some(vel_x),
                 Some(0.0),
                 Collider::round_cuboid(0.25, 0.05, 0.1),
                 None,
                 Bullet,
                 bullet_type,
                Option::None,
    );
}

pub fn spawn_strong_bullet(commands: &mut Commands, game_textures: &Res<GameTextures>, options: BulletOptions) {
    spawn_bullet(commands, game_textures.strong_laser.clone(), Weapon::StrongBullet, options, STRONG_BULLET_SPEED);
}

pub fn spawn_weak_bullet(commands: &mut Commands, game_textures: &Res<GameTextures>, options: BulletOptions) {
    spawn_bullet(commands, game_textures.weak_laser.clone(), Weapon::WeakBullet, options, WEAK_BULLET_SPEED);
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for bullet in bullets.iter() {
                if (*h1 == bullet
                    && !players.iter().any(|b| *h2 == b)
                    && !bullets.iter().any(|b| *h2 == b))
                    || (*h2 == bullet
                    && !players.iter().any(|b| *h1 == b)
                    && !bullets.iter().any(|b| *h1 == b))
                {
                    commands.entity(bullet).despawn_recursive();
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
