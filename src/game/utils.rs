use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY_SCALE_DEFAULT: f32 = 0.4;
const VELOCITY_DEFAULT: f32 = 0.0;

pub fn create_sprite_bundle(
    texture: Handle<Image>,
    (x_size, y_size): (f32, f32),
    (x_translation, y_translation, z_translation): (f32, f32, f32),
) -> SpriteBundle {
    SpriteBundle {
        texture,
        sprite: Sprite {
            custom_size: Some(Vec2::new(x_size, y_size)),
            ..default()
        },
        transform: Transform::from_xyz(x_translation, y_translation, z_translation),
        ..default()
    }
}

pub fn spawn_static_collider_object<T>(
    commands: &mut Commands,
    left_down: (f32, f32),
    right_up: (f32, f32),
    kind: T,
) -> Entity
where
    T: Component,
{
    let width = right_up.0 - left_down.0;
    let half_width = width / 2.;
    let height = right_up.1 - left_down.1;
    let half_height = height / 2.;

    commands
        .spawn()
        .insert(Transform::from_xyz(
            left_down.0 + half_width,
            left_down.1 + half_height,
            0.0,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(half_width, half_height))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(kind)
        .id()
}

pub fn spawn_dynamic_object(
    commands: &mut Commands,
    sprite: SpriteBundle,
    x_velocity: Option<f32>,
    gravity_scale: Option<f32>,
) -> Entity {
    commands
        .spawn_bundle(sprite)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Sleeping::disabled())
        .insert(GravityScale(gravity_scale.unwrap_or(GRAVITY_SCALE_DEFAULT)))
        .insert(Ccd::enabled())
        .insert(Velocity::linear(Vec2::new(
            x_velocity.unwrap_or(VELOCITY_DEFAULT),
            VELOCITY_DEFAULT,
        )))
        .id()
}

pub fn spawn_static_object(commands: &mut Commands, sprite: SpriteBundle) -> Entity {
    commands.spawn_bundle(sprite).insert(RigidBody::Fixed).id()
}

pub fn spawn_sensor_collider(
    commands: &mut Commands,
    entity: Entity,
    collider: Collider,
) -> Entity {
    commands
        .entity(entity)
        .insert(collider)
        .insert(Sensor(true))
        .id()
}

pub fn spawn_solid_collider(
    commands: &mut Commands,
    entity: Entity,
    collider: Collider,
    friction: Option<Friction>,
) -> Entity {
    commands
        .entity(entity)
        .insert(collider)
        .insert(friction.unwrap_or_default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id()
}
