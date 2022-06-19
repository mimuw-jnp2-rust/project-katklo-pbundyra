use bevy::prelude::*;
use rand::prelude::*;

use crate::game::monster::add_enemies;
use crate::game::powerups::add_powerups;
use crate::game::{CoffeeEvent, FinishLine, Level, RustEvent, Wall};
use crate::{AppState, GameTextures, Random};

use super::utils::*;

const BEGIN_WIDTH: usize = 10;
pub const SAFE_ZONE_WIDTH: usize = 5;
pub const GAME_WIDTH: usize = 150;
const MAP_WIDTH: usize = GAME_WIDTH + BEGIN_WIDTH;
const WALL_HEIGHT: f32 = 20.0;
const TILE_SIZE: f32 = 1.0;
const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(generate_map))
            .add_event::<CoffeeEvent>()
            .add_event::<RustEvent>();
    }
}

fn generate_map(
    mut rng: ResMut<Random>,
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    level: Res<Level>,
) {
    let world = create_world(&mut rng);
    add_floor(&mut commands, &game_textures, &world);
    add_start_and_finish_line(&mut commands, &game_textures, &world);
    add_enemies(&mut commands, &world, &game_textures, &mut rng, &level);
    add_powerups(&mut commands, &world, game_textures, &mut rng, &level);
}

fn create_world(rng: &mut ResMut<Random>) -> Vec<(i32, usize)> {
    let mut heights: Vec<(i32, usize)> = Vec::with_capacity(MAP_WIDTH);
    let mut height: usize = 0;

    // we want to start with a safe zone and some space on the left side of the player
    let beg = -(BEGIN_WIDTH as i32);
    (beg..SAFE_ZONE_WIDTH as i32).for_each(|i| {
        heights.push((i, height));
    });

    (SAFE_ZONE_WIDTH..GAME_WIDTH).for_each(|i| {
        heights.push((i as i32, height));
        height = get_next_height(rng, height)
    });

    heights
}

fn get_next_height(rng: &mut ResMut<Random>, current_height: usize) -> usize {
    let next_height = current_height as i32 + get_random_height_delta(rng);

    if next_height >= 0 {
        next_height as usize
    } else {
        0
    }
}

fn get_random_height_delta(rng: &mut ResMut<Random>) -> i32 {
    match rng.generator.gen_range(0..100) {
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    }
}

fn add_floor(commands: &mut Commands, game_textures: &Res<GameTextures>, world: &[(i32, usize)]) {
    add_sprites(commands, game_textures, world);
    add_colliders(world, commands);
}

fn add_sprites(commands: &mut Commands, game_textures: &Res<GameTextures>, world: &[(i32, usize)]) {
    world.iter().for_each(|&(x, height)| {
        add_column_of_tiles(
            commands,
            game_textures.floor.clone(),
            x as f32,
            0,
            height as i32,
        );
    });
}

fn add_column_of_tiles(
    commands: &mut Commands,
    texture: Handle<Image>,
    x: f32,
    y_min: i32,
    y_max: i32,
) {
    for h in y_min..=y_max {
        spawn_static_object(
            commands,
            create_sprite_bundle(texture.clone(), (TILE_SIZE, TILE_SIZE), (x, h as f32, 0.)),
        );
    }
}

// Add colliders for the whole map as big rectangles
fn add_colliders(world: &[(i32, usize)], commands: &mut Commands) {
    let (mut block_start, mut current_height) =
        world.first().map(|&(x, y)| (x, y)).unwrap_or((0, 0));

    world.iter().for_each(|&(x, height_at_x)| {
        if height_at_x != current_height {
            spawn_static_collider_object(
                commands,
                (block_start as f32 - HALF_TILE_SIZE, -HALF_TILE_SIZE),
                (
                    x as f32 - HALF_TILE_SIZE,
                    current_height as f32 + HALF_TILE_SIZE,
                ),
                Wall,
            );

            block_start = x;
            current_height = height_at_x;
        }
    });

    if let Some(last_x) = world.last().map(|&(x, _)| x) {
        spawn_static_collider_object(
            commands,
            (block_start as f32 - HALF_TILE_SIZE, -HALF_TILE_SIZE),
            (
                last_x as f32 + HALF_TILE_SIZE,
                current_height as f32 + HALF_TILE_SIZE,
            ),
            Wall,
        );
    }
}

fn add_start_and_finish_line(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    world: &[(i32, usize)],
) {
    let (start_x, start_y) = (-(BEGIN_WIDTH as f32), 0.);
    let (finish_x, finish_y) = world
        .last()
        .map(|&(x, y)| (x as f32 + 1., y as f32))
        .unwrap_or((0., 0.));
    add_column_of_tiles(
        commands,
        game_textures.floor.clone(),
        start_x,
        start_y as i32,
        (start_y + WALL_HEIGHT) as i32,
    );
    add_column_of_tiles(
        commands,
        game_textures.floor.clone(),
        finish_x,
        start_y as i32,
        finish_y as i32,
    );
    add_column_of_tiles(
        commands,
        game_textures.finish_line.clone(),
        finish_x,
        (finish_y + TILE_SIZE) as i32,
        (finish_y + WALL_HEIGHT) as i32,
    );

    spawn_static_collider_object(
        commands,
        (start_x - HALF_TILE_SIZE, start_y - HALF_TILE_SIZE),
        (
            start_x + HALF_TILE_SIZE,
            start_y + WALL_HEIGHT - HALF_TILE_SIZE,
        ),
        Wall,
    );

    spawn_static_collider_object(
        commands,
        (finish_x - HALF_TILE_SIZE, start_y - HALF_TILE_SIZE),
        (
            finish_x + HALF_TILE_SIZE,
            finish_y + WALL_HEIGHT - HALF_TILE_SIZE,
        ),
        FinishLine,
    );
}
