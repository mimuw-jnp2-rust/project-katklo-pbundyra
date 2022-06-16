use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{AppState, GameTextures};
use crate::game::boosters::add_boosters;
use crate::game::monster::add_enemies;
use super::utils::*;

const MAP_WIDTH: usize = 150;
const END_WALL_HEIGHT: usize = 20;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_map));
    }
}

fn spawn_map(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let world = create_world(MAP_WIDTH);
    add_sprites(&mut commands, &game_textures, &world);
    add_colliders(&world, &mut commands);
    add_enemies(&mut commands, &world, &game_textures);
    add_boosters(&mut commands, &world, game_textures);
}

fn create_world(width: usize) -> Vec<usize> {
    let mut heights: Vec<usize> = Vec::with_capacity(width);
    let mut height = 1;
    (0..width - 1).for_each(|_| {
        heights.push(height);
        height = get_next_height(height)
    });
    heights.push(height + END_WALL_HEIGHT);
    heights
}

fn add_sprites(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    world: &[usize],
) {
    world.iter().enumerate().for_each(|(x, height)| {
        add_tile(commands, game_textures, x as f32, *height);
    });
}

fn get_random_height_delta() -> i8 {
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    match random_number {
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    }
}

fn get_next_height(current_height: usize) -> usize {
    let next_height = current_height as i8 + get_random_height_delta();
    if next_height > 0 {
        next_height as usize
    } else {
        1
    }
}

fn add_tile(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    x: f32,
    height: usize,
) {
    for h in 0..height {
        commands.spawn_bundle(create_sprite_bundle(
            game_textures.floor.clone(),
            (1.0, 1.0),
            (x, h as f32 + 0.5, 0.),
        ));
    }
}

fn add_colliders(world: &[usize], commands: &mut Commands) {
    let max = match world.iter().max() {
        Some(m) => m,
        _ => panic!("add_colliders: World is empty"),
    };
    (1..=*max).for_each(|floor_height| {
        let mut start: Option<usize> = None;
        world
            .iter()
            .enumerate()
            .for_each(|(index, height_at_index)| {
                if *height_at_index >= floor_height && start.is_none() {
                    start = Some(index);
                } else if *height_at_index < floor_height && start.is_some() {
                    add_collider(commands, floor_height, *start.get_or_insert(0), index);
                    start = None
                }
            });

        if start.is_some() {
            add_collider(commands, floor_height, *start.get_or_insert(0), world.len());
        }
    })
}

fn add_collider(commands: &mut Commands, height: usize, from: usize, to: usize) {
    let width = to - from;
    let half_width = width as f32 / 2.;

    commands
        .spawn()
        .insert(Transform::from_xyz(
            from as f32 + half_width - 0.5,
            height as f32 - 0.5,
            0.0,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(half_width, 0.5));
}
