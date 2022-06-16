use bevy::prelude::*;
use bevy::window::PresentMode;

use game::GamePlugin;
use menu::MenuPlugin;

use crate::game::{GameTextures, MapPlugin, MonsterAiPlugin, PlayerPlugin};

pub mod utils;
mod game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    DeathMenu,
    EndMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Mario MIM!".to_string(),
            width: 640.0,
            height: 400.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0.71, 2.13, 2.44)))
        .add_state(AppState::MainMenu)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(MonsterAiPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTextures {
        player: asset_server.load("player.png"),
        weak_laser: asset_server.load("weak_laser.png"),
        strong_laser: asset_server.load("strong_laser.png"),
        bug: asset_server.load("bug.png"),
        coffee: asset_server.load("coffee.png"),
        rust: asset_server.load("rust.png"),
        floor: asset_server.load("floor.png"),
    });
}
