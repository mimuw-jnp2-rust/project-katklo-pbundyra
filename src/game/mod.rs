use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub use audio::*;
pub use bullets::*;
pub use camera::*;
pub use components::*;
pub use living_being::*;
pub use map::*;
pub use monster::*;
pub use moster_ai::*;
pub use player::*;
pub use powerups::*;
pub use utils::*;

use super::AppState;

mod audio;
mod bullets;
mod camera;
mod components;
mod living_being;
mod map;
mod monster;
mod moster_ai;
mod player;
mod powerups;
mod utils;

pub struct GameTextures {
    pub player: Handle<Image>,
    pub weak_laser: Handle<Image>,
    pub strong_laser: Handle<Image>,
    pub bug: Handle<Image>,
    pub coffee: Handle<Image>,
    pub rust: Handle<Image>,
    pub floor: Handle<Image>,
    pub finish_line: Handle<Image>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(back_to_main_menu_controls),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_all))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
    }
}

fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if *app_state.current() == AppState::InGame && keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        keys.reset(KeyCode::Escape);
    }
}

fn cleanup_all(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTextures {
        player: asset_server.load("images/player.png"),
        weak_laser: asset_server.load("images/weak_laser.png"),
        strong_laser: asset_server.load("images/strong_laser.png"),
        bug: asset_server.load("images/bug.png"),
        coffee: asset_server.load("images/coffee.png"),
        rust: asset_server.load("images/rust.png"),
        floor: asset_server.load("images/floor.png"),
        finish_line: asset_server.load("images/finish_line.png"),
    });
}

