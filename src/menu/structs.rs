use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButton {
    NewGame,
    Quit,
    MainMenu,
    SeedGenerate,
    InputButton,
    NextLevel,
    RestartLevel,
    RestartGame,
}

pub struct MenuTextures {
    pub play: Handle<Image>,
    pub exit: Handle<Image>,
    pub main: Handle<Image>,
    pub retry: Handle<Image>,
    pub font: Handle<Font>,
}

pub struct MenuData {
    pub menu_entity: Entity,
    pub camera_entity: Entity,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct InputText;

pub struct MenuColors {
    pub menu: UiColor,
    pub normal_button: UiColor,
    pub button_text: Color,
    pub button_hovered: UiColor,
    pub button_pressed: UiColor,
    pub button_selected: UiColor,
}

impl Default for MenuColors {
    fn default() -> Self {
        MenuColors {
            menu: Color::CRIMSON.into(),
            normal_button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_text: Color::rgb(0.9, 0.9, 0.9).into(),
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            button_selected: Color::rgb(0.35, 0.35, 0.35).into(),
        }
    }
}
