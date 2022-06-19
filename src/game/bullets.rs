use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::utils::*;
use super::GameDirection;
use crate::game::living_being::LivingBeingHitEvent;
use crate::game::{spawn_dynamic_object, Bullet, Enemy, Player, Weapon};
use crate::GameTextures;

pub struct ShootEvent;
pub struct FastShootEvent;

const WEAK_BULLET_SPEED: f32 = 8.25;
const STRONG_BULLET_SPEED: f32 = 18.5;

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
    bullet_type: Weapon,
    options: BulletOptions,
    def_vel: f32,
) {
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

    commands
        .entity(bullet_entity)
        .insert(Bullet)
        .insert(bullet_type);
}

pub fn spawn_strong_bullet(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    options: BulletOptions,
) {
    spawn_bullet(
        commands,
        game_textures.strong_laser.clone(),
        Weapon::StrongBullet,
        options,
        STRONG_BULLET_SPEED,
    );
}

pub fn spawn_weak_bullet(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    options: BulletOptions,
) {
    spawn_bullet(
        commands,
        game_textures.weak_laser.clone(),
        Weapon::WeakBullet,
        options,
        WEAK_BULLET_SPEED,
    );
}

pub fn destroy_bullet_on_contact(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            if let Ok(player) = players.get_single() {
                for bullet in bullets.iter() {
                    if (*ent1 == bullet && *ent2 != player) || (*ent2 == bullet && *ent1 != player)
                    {
                        commands.entity(bullet).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn kill_enemy(
    bullets: Query<Entity, With<Bullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_event: EventReader<CollisionEvent>,
    mut send_hit_event: EventWriter<LivingBeingHitEvent>,
) {
    for collision_event in collision_event.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            for bullet in bullets.iter() {
                for enemy in enemies.iter() {
                    if (*ent1 == bullet && *ent2 == enemy) || (*ent1 == enemy && *ent2 == bullet) {
                        send_hit_event.send(LivingBeingHitEvent { entity: enemy });
                    }
                }
            }
        }
    }
}
