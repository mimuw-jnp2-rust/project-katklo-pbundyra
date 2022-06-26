use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::bullets::{spawn_strong_bullet, spawn_weak_bullet, BulletOptions};
use crate::game::monster::death_by_enemy;
use crate::game::{
    camera_follow_player, AudioAssets, FinishLine, GameDirection, LastDespawnedEntity,
    PhantomEntity, SimpleAudioEvent, Wall, Weapon, COFFEE_DURATION, RUST_DURATION,
};
use crate::GameTextures;

use super::super::AppState;
use super::camera::new_camera_2d;
use super::components::Jumper;
use super::utils::*;

pub struct PlayerPlugin;

pub struct DeadPlayerEvent;

const PLAYER_NORMAL_SPEED: f32 = 8.0;
const PLAYER_INCREASE_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub weapon: Weapon,
    pub direction: GameDirection,
    pub weapon_upgrade_timer: Timer,
    pub coffee_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: PLAYER_NORMAL_SPEED,
            weapon: Weapon::WeakBullet,
            direction: GameDirection::Right,
            weapon_upgrade_timer: Timer::new(Duration::from_secs(0), false),
            coffee_timer: Timer::new(Duration::from_secs(0), false),
        }
    }
}

impl Player {
    pub fn spawn(commands: &mut Commands, game_textures: Res<GameTextures>) {
        let mut player_entity = spawn_dynamic_object(
            commands,
            create_sprite_bundle(
                game_textures.player.clone(),
                Vec2::new(0.9, 0.9),
                Vec3::new(0.0, 2.0, 0.0),
            ),
            None,
            None,
        );
        player_entity = spawn_solid_collider(
            commands,
            player_entity,
            Collider::round_cuboid(0.25, 0.3, 0.05),
            Some(Friction::coefficient(3.)),
        );
        commands
            .entity(player_entity)
            .insert(Player::default())
            .insert(Jumper::default());
    }

    pub fn increase_speed(&mut self) {
        self.coffee_timer = Timer::new(Duration::from_secs(COFFEE_DURATION), false);
        self.speed = PLAYER_INCREASE_SPEED;
    }

    pub fn decrease_speed(&mut self) {
        self.speed = PLAYER_NORMAL_SPEED;
    }

    pub fn degrade_weapon(&mut self) {
        self.weapon = Weapon::WeakBullet;
    }

    pub fn upgrade_weapon(&mut self) {
        self.weapon = Weapon::StrongBullet;
        self.weapon_upgrade_timer = Timer::new(Duration::from_secs(RUST_DURATION), false);
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_jumps)
                    .with_system(player_movement)
                    .with_system(jump_reset)
                    .with_system(finish)
                    .with_system(death_by_enemy)
                    .with_system(camera_follow_player)
                    .with_system(fire_controller)
                    .with_system(handle_death),
            )
            .add_event::<DeadPlayerEvent>();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    phantom_entity: Query<Entity, With<PhantomEntity>>,
) {
    Player::spawn(&mut commands, game_textures);
    commands.spawn_bundle(new_camera_2d());
    for entity in phantom_entity.iter() {
        commands.insert_resource(LastDespawnedEntity { entity });
    }
}

pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>,
) {
    if let Ok((mut jumper, mut velocity)) = players.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse);
            jumper.is_jumping = true
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &mut Velocity)>,
) {
    if let Ok((mut player, mut velocity)) = players.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            player.direction = GameDirection::Left;
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.direction = GameDirection::Right;
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y);
        }
    }
}

pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    positions: Query<(&mut Transform, &RigidBody, &mut Player, &mut Velocity), With<Player>>,
    mut audio_event_sender: EventWriter<SimpleAudioEvent>,
    audio_assets: Res<AudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (pos, _, player, vel) in positions.iter() {
            let options = BulletOptions {
                x: pos.translation.x,
                y: pos.translation.y,
                direction: player.direction,
                player_vex: vel.linvel.x,
            };
            match player.weapon {
                Weapon::WeakBullet => {
                    audio_event_sender.send(SimpleAudioEvent {
                        audio_src: audio_assets.shoot.clone(),
                    });
                    spawn_weak_bullet(&mut commands, &game_textures, options);
                }
                Weapon::StrongBullet => {
                    audio_event_sender.send(SimpleAudioEvent {
                        audio_src: audio_assets.fast_shoot.clone(),
                    });
                    spawn_strong_bullet(&mut commands, &game_textures, options);
                }
            }
        }
    }
}

pub fn jump_reset(
    mut jumpers: Query<(Entity, &mut Jumper), With<Player>>,
    walls: Query<Entity, With<Wall>>,
    players: Query<Entity, With<Player>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        if let Ok((_, mut jumper)) = jumpers.get_single_mut() {
            if let CollisionEvent::Started(ent1, ent2, _) = collision_event {
                if get_both_proper_entities(ent1, ent2, &walls, &players).is_ok() {
                    jumper.is_jumping = false;
                }
            }
        }
    }
}

fn handle_death(
    mut state: ResMut<State<AppState>>,
    mut dead_player_events: EventReader<DeadPlayerEvent>,
    mut audio_event_sender: EventWriter<SimpleAudioEvent>,
    audio_assets: Res<AudioAssets>,
) {
    dead_player_events.iter().for_each(|_| {
        state
            .replace(AppState::FailMenu)
            .expect("Could not set state to DeathMenu");
        audio_event_sender.send(SimpleAudioEvent {
            audio_src: audio_assets.death.clone(),
        });
    });
}

pub fn finish(
    players: Query<Entity, With<Player>>,
    lines: Query<Entity, With<FinishLine>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut state: ResMut<State<AppState>>,
) {
    for contact_event in contact_events.iter() {
        if let CollisionEvent::Started(ent1, ent2, _) = contact_event {
            let from_collision = get_both_proper_entities(ent1, ent2, &players, &lines);

            if from_collision.is_ok() {
                state.replace(AppState::WinMenu).unwrap()
            }
        }
    }
}
