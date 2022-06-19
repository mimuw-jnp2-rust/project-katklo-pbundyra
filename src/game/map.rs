use bevy::prelude::*;
use rand::prelude::*;

use crate::{AppState, GameTextures, Random};
use crate::game::{CoffeeEvent, FinishLine, RustEvent, Wall};
use crate::game::powerups::add_powerups;
use crate::game::monster::add_enemies;

use super::utils::*;

const BEGIN_WIDTH: usize = 10;
pub const SAFE_ZONE_WIDTH: usize = 5;
pub const GAME_WIDTH: usize = 150;
const MAP_WIDTH: usize = GAME_WIDTH + BEGIN_WIDTH + 1;
const WALL_HEIGHT: f32 = 20.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_map))
            .add_event::<CoffeeEvent>()
            .add_event::<RustEvent>();
    }
}

fn spawn_map(
    mut rng: ResMut<Random>,
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let world = create_world(&mut rng);
    add_sprites(&mut commands, &game_textures, &world);
    add_colliders(&world, &mut commands);
    add_finish_line(&mut commands, &game_textures, &world);
    add_enemies(&mut commands, &world, &game_textures, &mut rng);
    add_powerups(&mut commands, &world, game_textures, &mut rng);
}

fn create_world(rng: &mut ResMut<Random>) -> Vec<(i32, usize)> {
    let mut heights: Vec<(i32, usize)> = Vec::with_capacity(MAP_WIDTH);
    let mut height: usize = 0;
    let beg = -(BEGIN_WIDTH as i32);

    heights.push((beg - 1, WALL_HEIGHT as usize));

    (beg..SAFE_ZONE_WIDTH as i32).for_each(|i| {
        heights.push((i, height));
    });

    (SAFE_ZONE_WIDTH..GAME_WIDTH).for_each(|i| {
        heights.push((i as i32, height));
        height = get_next_height(rng, height)
    });

    heights
}

fn add_sprites(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    world: &[(i32, usize)],
) {
    world.iter().for_each(|&(x, height)| {
        add_tile(commands, game_textures, x as f32, height);
    });
}

fn get_random_height_delta(rng: &mut ResMut<Random>) -> i8 {
    let random_number: u32 = rng.generator.gen_range(0..100);
    match random_number {
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    }
}

fn get_next_height(rng: &mut ResMut<Random>, current_height: usize) -> usize {
    let next_height = current_height as i8 + get_random_height_delta(rng);
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

fn add_colliders(world: &[(i32, usize)], commands: &mut Commands) {
    let max = match world.iter().map(|&(_, y)| y).max() {
        Some(m) => m,
        _ => panic!("add_colliders: World is empty"),
    };

    (0..=max).for_each(|height| {
        let mut start: Option<i32> = None;
        let floor_height = height as f32;
        world
            .iter()
            .for_each(|&(index, height_at_index)| {
                if height_at_index >= height && start.is_none() {
                    start = Some(index);
                } else if height_at_index < height {
                    if let Some(s) = start {
                        spawn_static_collider(commands, (s as f32 - 0.5, floor_height - 0.5), (index as f32 - 0.5, floor_height + 0.5), Wall);
                        start = None
                    }
                }
            });

        if let Some(s) = start {
            spawn_static_collider(commands, (s as f32 - 0.5, floor_height - 0.5), (GAME_WIDTH as f32 - 0.5, floor_height + 0.5), Wall);
        }
    })
}

fn add_finish_line(commands: &mut Commands, game_textures: &Res<GameTextures>, world: &[(i32, usize)]) {
    let (finish_x_position, last_height) = world.last().map(|&(x, y)| (x as f32, y as f32)).unwrap_or((0., 0.));

    for h in 0..=WALL_HEIGHT as usize {
        commands.spawn_bundle(create_sprite_bundle(
            game_textures.finish_line.clone(),
            (1.0, 1.0),
            (finish_x_position, last_height + h as f32 + 1., 0.),
        ));
    }
    spawn_static_collider(commands, (finish_x_position - 0.5, last_height), (finish_x_position + 0.5, last_height + WALL_HEIGHT), FinishLine);
}

