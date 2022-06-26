use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::utils::*;
use super::GameDirection;
use crate::game::{
    spawn_dynamic_object, AudioAssets, Bullet, ComplexAudioEvent, Enemy, EnemyBullet,
    PlayersBullet, Wall,
};
use crate::GameTextures;

pub struct BulletsPlugin;

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

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(destroy_bullet_on_contact)
                .with_system(kill_enemy),
        )
        .add_event::<ShootEvent>()
        .add_event::<FastShootEvent>();
    }
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
        create_sprite_bundle(
            texture,
            Vec2::new(0.5, 0.2),
            Vec3::new(options.x + spawn_x, options.y, 0.0),
        ),
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
            let from_collision = get_both_proper_entities(ent1, ent2, &walls, &bullets);

            if let Ok((_, bullet)) = from_collision {
                commands.entity(bullet).despawn_recursive()
            }
        }
    }
}

pub fn kill_enemy(
    mut commands: Commands,
    bullets: Query<Entity, With<PlayersBullet>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut audio_event_sender: EventWriter<ComplexAudioEvent>,
    audio_assets: Res<AudioAssets>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
            let from_collision = get_both_proper_entities(ent1, ent2, &bullets, &enemies);

            if let Ok((bullet, enemy)) = from_collision {
                audio_event_sender.send(ComplexAudioEvent {
                    audio_src: audio_assets.hits.clone(),
                });
                commands.entity(bullet).despawn_recursive();
                commands.entity(enemy).despawn_recursive();
            }
        }
    }
}
