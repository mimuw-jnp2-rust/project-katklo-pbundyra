use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub use camera::*;
pub use components::*;
pub use map::*;
pub use moster_ai::*;
pub use player::*;

use super::AppState;

mod boosters;
mod bullets;
mod camera;
mod components;
mod map;
mod monster;
mod moster_ai;
mod player;
mod utils;

pub struct GameTextures {
    pub player: Handle<Image>,
    pub weak_laser: Handle<Image>,
    pub strong_laser: Handle<Image>,
    pub bug: Handle<Image>,
    pub coffee: Handle<Image>,
    pub rust: Handle<Image>,
    pub floor: Handle<Image>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
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
