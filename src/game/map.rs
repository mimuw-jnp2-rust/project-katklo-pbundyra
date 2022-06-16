use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{AppState, GameTextures};
use crate::game::{FinishLine, Wall};
use crate::game::boosters::add_boosters;
use crate::game::monster::add_enemies;
use super::utils::*;

pub const SAFE_ZONE_WIDTH: usize = 20;
pub const GAME_WIDTH: usize = 150;
const MAP_WIDTH: usize = GAME_WIDTH + SAFE_ZONE_WIDTH;
const WALL_HEIGHT: f32 = 20.0;

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
    let world = create_world();
    add_sprites(&mut commands, &game_textures, &world);
    add_colliders(&world, &mut commands);
    add_finish_line(&mut commands, &game_textures, &world);
    add_enemies(&mut commands, &world, &game_textures);
    add_boosters(&mut commands, &world, game_textures);
}

fn create_world() -> Vec<usize> {
    let mut heights: Vec<usize> = Vec::with_capacity(MAP_WIDTH);
    let mut height: usize = 0;
    (0..MAP_WIDTH).for_each(|_| {
        heights.push(height);
        height = get_next_height(height)
    });

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
    if next_height >= 0 {
        next_height as usize
    } else {
        0
    }
}

fn add_tile(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    x: f32,
    height: usize,
) {
    for h in 0..=height {
        commands.spawn_bundle(create_sprite_bundle(
            game_textures.floor.clone(),
            (1.0, 1.0),
            (x, h as f32, 0.),
        ));
    }
}

fn add_colliders(world: &[usize], commands: &mut Commands) {
    let max = match world.iter().max() {
        Some(m) => m,
        _ => panic!("add_colliders: World is empty"),
    };

    (0..=*max).for_each(|height| {
        let mut start: Option<usize> = None;
        let floor_height = height as f32;
        world
            .iter()
            .enumerate()
            .for_each(|(index, height_at_index)| {
                if *height_at_index >= height && start.is_none() {
                    start = Some(index);
                } else if *height_at_index < height {
                    if let Some(s) = start {
                        add_collider(commands, (s as f32 - 0.5, floor_height - 0.5), (index as f32 - 0.5, floor_height + 0.5), Wall);
                        start = None
                    }
                }
            });

        if let Some(s) = start {
            add_collider(commands, (s as f32 - 0.5, floor_height - 0.5), (world.len() as f32 - 0.5, floor_height + 0.5), Wall);
        }
    })
}

fn add_collider<T>(commands: &mut Commands, left_down: (f32, f32), right_up: (f32, f32), kind: T) where T: Component {
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
        .insert(kind);
}

fn add_finish_line(commands: &mut Commands, game_textures: &Res<GameTextures>, world: &[usize]) {
    let last_height = world.last().map(|h| *h as f32).unwrap_or(0.0);
    let finish_x_position = world.len() as f32 - 1.0;

    for h in 0..=WALL_HEIGHT as usize {
        commands.spawn_bundle(create_sprite_bundle(
            game_textures.finish.clone(),
            (1.0, 1.0),
            (finish_x_position, last_height + h as f32 + 1., 0.),
        ));
    }

    add_collider(commands, (finish_x_position - 0.5, last_height), (finish_x_position + 0.5, last_height + WALL_HEIGHT), FinishLine);
}

