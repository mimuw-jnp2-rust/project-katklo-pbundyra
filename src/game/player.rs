use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{camera_follow_player, FastShootEvent, FinishLine, GameDirection, ShootEvent, Weapon};
use crate::game::powerups::{drink_coffee, learn_rust};
use crate::game::bullets::{BulletOptions, destroy_bullet_on_contact, kill_enemy, spawn_strong_bullet, spawn_weak_bullet};
use crate::game::living_being::{LivingBeingDeathEvent, LivingBeingHitEvent, on_living_being_dead, on_living_being_hit};
use crate::game::monster::death_by_enemy;
use crate::GameTextures;

use super::camera::new_camera_2d;
use super::components::{LivingBeing, Jumper};
use super::super::AppState;
use super::utils::*;

pub struct PlayerPlugin;

pub struct DeadPlayerEvent {
    pub entity: Entity,
}

const SHOOTING_TIMESTEP: f64 = 0.1;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub weapon: Weapon,
    pub direction: GameDirection,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: 7.0,
            weapon: Weapon::WeakBullet,
            direction: GameDirection::Right,
        }
    }
}

#[allow(dead_code)]
impl Player {
    pub fn spawn(commands: &mut Commands, game_textures: Res<GameTextures>) {
        spawn_object(commands,
                     create_sprite_bundle(game_textures.player.clone(),
                                          (0.9, 1.5),
                                          (0.0, 2.0, 0.0)),
                     None,
                     None,
                     Collider::round_cuboid(0.2, 0.2, 0.1),
                     Some(Friction::coefficient(3.)),
                     Player::default(),
                     LivingBeing::default(),
        );
    }

    pub fn change_weapon(&mut self) {
        match self.weapon {
            Weapon::WeakBullet => self.weapon = Weapon::StrongBullet,
            Weapon::StrongBullet => self.weapon = Weapon::WeakBullet,
        }
    }

    pub fn increase_speed(&mut self) {
        self.speed += 0.25
    }

    pub fn decrease_speed(&mut self) {
        self.speed -= 0.25;
    }

    pub fn degrade_weapon(&mut self) {
        self.weapon = Weapon::WeakBullet;
    }

    pub fn powerup_weapon(&mut self) {
        self.weapon = Weapon::StrongBullet;
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
                    .with_system(destroy_bullet_on_contact)
                    .with_system(death_by_enemy)
                    .with_system(camera_follow_player)
                    .with_system(kill_enemy)
                    .with_system(changing_weapon)
                    .with_system(drink_coffee)
                    .with_system(learn_rust)
                    .with_system(on_living_being_dead)
                    .with_system(fire_controller)
                    .with_system(on_living_being_hit)
            )
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(FixedTimestep::step(SHOOTING_TIMESTEP))
            // )
            .add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>()
            .add_event::<DeadPlayerEvent>()
            .add_event::<ShootEvent>()
            .add_event::<FastShootEvent>();
    }
}

pub fn spawn_player(mut commands: Commands, game_textures: Res<GameTextures>) {
    Player::spawn(&mut commands, game_textures);
    commands.spawn_bundle(new_camera_2d());
}

pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
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
    for (mut player, mut velocity) in players.iter_mut() {
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

pub fn changing_weapon(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut Player, With<Player>>,
) {
    if let Ok(mut player) = players.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::S) {
            player.change_weapon();
        }
    }
}

pub fn fire_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut game_textures: Res<GameTextures>,
    positions: Query<(&mut Transform, &RigidBody, &mut Player, &mut Velocity), With<Player>>,
    mut send_shoot_event: EventWriter<ShootEvent>,
    mut send_fast_shoot_event: EventWriter<FastShootEvent>,
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
                    send_shoot_event.send(ShootEvent);
                    spawn_weak_bullet(&mut commands, &mut game_textures, options);
                }
                Weapon::StrongBullet => {
                    send_fast_shoot_event.send(FastShootEvent);
                    spawn_strong_bullet(&mut commands, &mut game_textures, options);
                }
            }
        }
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _) = collision_event {
                if *h1 == entity || *h2 == entity {
                    jumper.is_jumping = false
                }
            }
        }
    }
}

// TODO zrobić lepiej przy okazji collision eventow
pub fn finish(
    mut players: Query<(Entity, &mut Player)>,
    mut lines: Query<(Entity, &mut FinishLine)>,
    mut contact_events: EventReader<CollisionEvent>,
    mut state: ResMut<State<AppState>>,
) {
    for contact_event in contact_events.iter() {
        for (entity1, _) in players.iter_mut() {
            for (entity2, _) in lines.iter_mut() {
                if let CollisionEvent::Started(h1, h2, _) = contact_event {
                    if (*h1 == entity1 && *h2 == entity2) || (*h1 == entity2 && *h2 == entity1) {
                        state.set(AppState::EndMenu).unwrap();
                    }
                }
            }
        }
    }
}
