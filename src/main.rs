use bevy::prelude::*;
use bevy::window::PresentMode;

use game::GamePlugin;
use menu::MenuPlugin;

use crate::game::{
    BulletsPlugin, GameAudioPlugin, GameTextures, Level, MapPlugin, MonsterAiPlugin, PlayerPlugin,
    PowerupsPlugin, Random,
};

mod game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    MainMenu,
    FailMenu,
    WinMenu,
    StopMenu,
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
        .insert_resource(ClearColor(Color::BEIGE))
        .add_state(AppState::MainMenu)
        .add_plugin(GameAudioPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(BulletsPlugin)
        .add_plugin(PowerupsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(MonsterAiPlugin)
        .run();
}
