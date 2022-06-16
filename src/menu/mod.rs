use bevy::{app::AppExit, prelude::*};

use crate::AppState;
use crate::menu::utils::{button_system, cleanup_menu, MenuMaterials, setup};

mod utils;

pub struct MenuPlugin;

#[derive(Component)]
pub enum MenuButton {
    Play,
    Quit,
    MainMenu,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuMaterials>()
            .add_system(button_press_system)
            .add_system(button_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::DeathMenu).with_system(setup_death_menu))
            .add_system_set(SystemSet::on_exit(AppState::DeathMenu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(AppState::EndMenu).with_system(setup_end_menu))
            .add_system_set(SystemSet::on_exit(AppState::EndMenu).with_system(cleanup_menu));
    }
}

fn setup_main_menu(commands: Commands,
                   asset_server: Res<AssetServer>,
                   materials: Res<MenuMaterials>) {
    setup(commands, asset_server, materials, vec![("New game", MenuButton::Play), ("Quit", MenuButton::Quit)]);
}

fn setup_death_menu(commands: Commands,
                    asset_server: Res<AssetServer>,
                    materials: Res<MenuMaterials>) {
    setup(commands, asset_server, materials, vec![("Try again", MenuButton::Play), ("Go to main menu", MenuButton::MainMenu)]);
}

fn setup_end_menu(commands: Commands,
                  asset_server: Res<AssetServer>,
                  materials: Res<MenuMaterials>) {
    setup(commands, asset_server, materials, vec![("Play again", MenuButton::Play), ("Go to main menu", MenuButton::MainMenu)]);
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton, Changed<Interaction>), With<Button>>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, _) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::InGame)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::Quit => exit.send(AppExit),
                MenuButton::MainMenu => state
                    .set(AppState::MainMenu)
                    .expect("Couldn't switch state to InGame"),
            };
        }
    }
}
