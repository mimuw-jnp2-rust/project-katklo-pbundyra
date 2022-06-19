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
    pub positive: Vec<Handle<Image>>,
    pub negative: Vec<Handle<Image>>,
    pub neutral: Vec<Handle<Image>>,
    pub font: Handle<Font>,
}

impl MenuTextures {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        MenuTextures {
            play: asset_server.load("menu/play.png"),
            exit: asset_server.load("menu/exit.png"),
            main: asset_server.load("menu/main.png"),
            retry: asset_server.load("menu/retry.png"),
            positive: vec![asset_server.load("menu/positive1.png"),
                           asset_server.load("menu/positive2.png"),
                           asset_server.load("menu/positive3.png"),
                           asset_server.load("menu/positive4.png"),
                           asset_server.load("menu/positive5.png"),
                           asset_server.load("menu/positive6.png"),
            ],
            negative: vec![asset_server.load("menu/negative1.png"),
                           asset_server.load("menu/negative2.png"),
                           asset_server.load("menu/negative3.png"),
                           asset_server.load("menu/negative4.png"),
                           asset_server.load("menu/negative5.png"),
                           asset_server.load("menu/negative6.png"),
                           asset_server.load("menu/negative7.png"),
                           asset_server.load("menu/negative8.png"),
                           asset_server.load("menu/negative9.png"),
                           asset_server.load("menu/negative10.png"),
                           asset_server.load("menu/negative11.png"),
                           asset_server.load("menu/negative12.png"),
                           asset_server.load("menu/negative13.png"),
            ],
            neutral: vec![asset_server.load("menu/neutral1.png")],
            font: asset_server.load("fonts/FiraSans-LightItalic.ttf"),
        }
    }
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
