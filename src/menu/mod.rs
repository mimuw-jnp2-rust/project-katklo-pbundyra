use bevy::prelude::*;

use crate::menu::structs::{MenuButton, MenuColors, MenuTextures};
use crate::menu::systems::{
    button_press_system, button_system, input_button_system, read_input_system, text_update_system,
};
use crate::menu::utils::{cleanup_menu, setup_level_end, setup_main};
use crate::{AppState, Level, Random};

mod structs;
mod systems;
mod utils;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuColors>()
            .add_startup_system(menu_setup)
            .add_system(button_press_system)
            .add_system(button_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(input_button_system)
                    .with_system(text_update_system)
                    .with_system(read_input_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::StopMenu).with_system(setup_stop_menu))
            .add_system_set(SystemSet::on_exit(AppState::StopMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::FailMenu).with_system(setup_fail_menu))
            .add_system_set(SystemSet::on_exit(AppState::FailMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::WinMenu).with_system(setup_win_menu))
            .add_system_set(SystemSet::on_exit(AppState::WinMenu).with_system(cleanup_menu));
    }
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuTextures::load(asset_server));
}

fn setup_main_menu(
    mut random: ResMut<Random>,
    commands: Commands,
    colors: Res<MenuColors>,
    textures: Res<MenuTextures>,
) {
    *random = Random::new();
    setup_main(
        commands,
        colors,
        textures,
        "Mario MIM",
        vec![
            ("New game", MenuButton::NewGame),
            ("Quit", MenuButton::Quit),
        ],
    );
}

fn setup_fail_menu(
    commands: Commands,
    colors: Res<MenuColors>,
    textures: Res<MenuTextures>,
    level: Res<Level>,
) {
    setup_level_end(
        commands,
        colors,
        textures,
        vec![
            ("Try again level", MenuButton::RestartLevel),
            ("Start from beginning", MenuButton::RestartGame),
            ("Go to main menu", MenuButton::MainMenu),
        ],
        Some(false),
        level.level,
    );
}

fn setup_win_menu(
    commands: Commands,
    colors: Res<MenuColors>,
    textures: Res<MenuTextures>,
    level: Res<Level>,
) {
    setup_level_end(
        commands,
        colors,
        textures,
        vec![
            ("Next level", MenuButton::NextLevel),
            ("Go to main menu", MenuButton::MainMenu),
        ],
        Some(true),
        level.level,
    );
}

fn setup_stop_menu(
    commands: Commands,
    colors: Res<MenuColors>,
    textures: Res<MenuTextures>,
    level: Res<Level>,
) {
    setup_level_end(
        commands,
        colors,
        textures,
        vec![
            ("Resume", MenuButton::Resume),
            ("Restart level", MenuButton::RestartLevel),
            ("Restart game", MenuButton::RestartGame),
            ("Go to main menu", MenuButton::MainMenu),
        ],
        None,
        level.level,
    );
}
