use bevy::{prelude::*};

use crate::{AppState, Random};
use crate::menu::structs::{MenuButton, MenuColors, MenuTextures};
use crate::menu::systems::{button_press_system, button_system, input_button_system, read_input_system, text_update_system};
use crate::menu::utils::{cleanup_menu, setup, setup_with_input};

mod utils;
mod systems;
mod structs;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuColors>()
            .add_startup_system(menu_setup)
            .add_system(button_press_system)
            .add_system(button_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu)
                .with_system(input_button_system)
                .with_system(text_update_system)
                .with_system(read_input_system))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::DeathMenu).with_system(setup_death_menu))
            .add_system_set(SystemSet::on_exit(AppState::DeathMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::EndMenu).with_system(setup_end_menu))
            .add_system_set(SystemSet::on_exit(AppState::EndMenu).with_system(cleanup_menu));
    }
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuTextures {
        play: asset_server.load("menu/play.png"),
        exit: asset_server.load("menu/exit.png"),
        main: asset_server.load("menu/main.png"),
        retry: asset_server.load("menu/retry.png"),
        font: asset_server.load("fonts/FiraSans-LightItalic.ttf"),
    });
}

fn setup_main_menu(mut random: ResMut<Random>, commands: Commands,
                   colors: Res<MenuColors>, textures: Res<MenuTextures>) {
    *random = Random::new();
    setup_with_input(commands, colors, textures, "Mario MIM", vec![("New game", MenuButton::NewGame), ("Quit", MenuButton::Quit)]);
}

fn setup_death_menu(commands: Commands, colors: Res<MenuColors>, textures: Res<MenuTextures>) {
    setup(commands, colors, textures, "Segmentation fault (core dumped)", vec![("Try again level", MenuButton::RestartLevel), ("Start from beginning", MenuButton::RestartGame), ("Go to main menu", MenuButton::MainMenu)]);
}

fn setup_end_menu(commands: Commands, colors: Res<MenuColors>, textures: Res<MenuTextures>) {
    setup(commands, colors, textures, "Process finished with exit code 0", vec![("Next level", MenuButton::NextLevel), ("Go to main menu", MenuButton::MainMenu)]);
}

