use bevy::prelude::*;

use crate::menu::structs::{InputText, MenuButton, MenuColors, MenuData, MenuTextures};
use crate::{AppState, Level, Random};

pub fn setup(
    mut commands: Commands,
    colors: Res<MenuColors>,
    textures: Res<MenuTextures>,
    text: &'static str,
    buttons: Vec<(&'static str, MenuButton)>,
) {
    let menu_entity = commands
        .spawn_bundle(menu_bundle(&colors))
        .with_children(|parent| {
            parent.spawn_bundle(main_text_bundle(&colors, &textures, text));

            for (text, but) in buttons {
                spawn_button(&colors, &textures, parent, text, but);
            }
        })
        .id();

    insert_menu_data(commands, menu_entity);
}

pub fn setup_with_input(
    mut commands: Commands,
    colors: Res<MenuColors>,
    textures: Res<MenuTextures>,
    text: &'static str,
    buttons: Vec<(&'static str, MenuButton)>,
) {
    let menu_entity = commands
        .spawn_bundle(menu_bundle(&colors))
        .with_children(|parent| {
            parent.spawn_bundle(main_text_bundle(&colors, &textures, text));

            parent.spawn_bundle(info_text_bundle(
                &colors,
                &textures,
                "Enter seed for random generator:",
            ));
            spawn_input_button(&colors, &textures, parent);
            spawn_button(
                &colors,
                &textures,
                parent,
                "generate seed",
                MenuButton::SeedGenerate,
            );

            for (text, but) in buttons {
                spawn_button(&colors, &textures, parent, text, but);
            }
        })
        .id();

    insert_menu_data(commands, menu_entity);
}

fn insert_menu_data(mut commands: Commands, menu_entity: Entity) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    commands.insert_resource(MenuData {
        menu_entity,
        camera_entity,
    });
}

fn spawn_button(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    parent: &mut ChildBuilder,
    text: &'static str,
    but: MenuButton,
) {
    parent
        .spawn_bundle(button_bundle(&colors))
        .with_children(|parent| {
            match but {
                MenuButton::NewGame | MenuButton::NextLevel => {
                    parent.spawn_bundle(button_icon_bundle(textures.play.clone()));
                }
                MenuButton::RestartLevel | MenuButton::RestartGame => {
                    parent.spawn_bundle(button_icon_bundle(textures.retry.clone()));
                }
                MenuButton::Quit => {
                    parent.spawn_bundle(button_icon_bundle(textures.exit.clone()));
                }
                MenuButton::MainMenu => {
                    parent.spawn_bundle(button_icon_bundle(textures.main.clone()));
                }
                _ => {}
            };

            parent.spawn_bundle(button_text_bundle(&colors, &textures, text));
        })
        .insert(but);
}

fn spawn_input_button(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    parent: &mut ChildBuilder,
) {
    parent
        .spawn_bundle(button_bundle(&colors))
        .with_children(|parent| {
            parent
                .spawn_bundle(button_text_bundle(&colors, &textures, ""))
                .insert(InputText);
        })
        .insert(MenuButton::InputButton);
}

fn menu_bundle(materials: &Res<MenuColors>) -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: Rect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        },
        color: materials.menu,
        ..default()
    }
}

fn button_bundle(materials: &Res<MenuColors>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(410.0), Val::Px(65.0)),
            margin: Rect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: materials.normal_button,
        ..default()
    }
}

fn button_text_bundle(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    label: &str,
) -> TextBundle {
    text_bundle(colors, textures, label, 40., None)
}

fn main_text_bundle(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    label: &str,
) -> TextBundle {
    text_bundle(
        colors,
        textures,
        label,
        80.,
        Some(Style {
            margin: Rect::all(Val::Px(50.0)),
            ..default()
        }),
    )
}

fn info_text_bundle(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    label: &str,
) -> TextBundle {
    text_bundle(colors, textures, label, 25., None)
}

fn text_bundle(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    label: &str,
    size: f32,
    style: Option<Style>,
) -> TextBundle {
    TextBundle {
        style: style.unwrap_or(Style::default()),
        text: Text::with_section(
            label,
            TextStyle {
                font: textures.font.clone(),
                font_size: size,
                color: colors.button_text,
            },
            Default::default(),
        ),
        ..default()
    }
}

fn button_icon_bundle(icon: Handle<Image>) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(30.0), Val::Auto),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(10.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..default()
        },
        image: UiImage(icon),
        ..default()
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_entity).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

pub fn start_game_for_level(
    state: &mut ResMut<State<AppState>>,
    rng: &mut ResMut<Random>,
    level: &mut ResMut<Level>,
) {
    rng.make_generator_for_level(level.level);

    state
        .set(AppState::InGame)
        .expect("Couldn't switch state to InGame")
}

pub fn start_new_game(
    state: &mut ResMut<State<AppState>>,
    rng: &mut ResMut<Random>,
    level: &mut ResMut<Level>,
) {
    if rng.seed.is_empty() {
        rng.new_random_seed();
    }

    level.level = 1;
    start_game_for_level(state, rng, level);
}
