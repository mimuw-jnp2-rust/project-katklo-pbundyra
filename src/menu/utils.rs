use bevy::prelude::*;

use crate::menu::MenuButton;
use crate::MenuTextures;

pub struct MenuData {
    menu_entity: Entity,
    camera_entity: Entity,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

pub struct MenuColors {
    menu: UiColor,
    button: UiColor,
    button_text: Color,
    button_hovered: UiColor,
    button_pressed: UiColor,
}

impl Default for MenuColors {
    fn default() -> Self {
        MenuColors {
            menu: Color::CRIMSON.into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_text: Color::rgb(0.9, 0.9, 0.9).into(),
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
        }
    }
}


pub fn button_system(
    mut buttons: Query<(&Interaction, &mut UiColor, Changed<Interaction>), With<Button>>,
    materials: Res<MenuColors>,
) {
    for (interaction, mut color, _) in buttons.iter_mut() {
        *color = match *interaction {
            Interaction::Clicked => materials.button_pressed,
            Interaction::Hovered => materials.button_hovered,
            Interaction::None => materials.button,
        }
    }
}

pub fn setup(mut commands: Commands,
             colors: Res<MenuColors>,
             textures: Res<MenuTextures>,
             text: &'static str,
             buttons: Vec<(&'static str, MenuButton)>) {
    let menu_entity = commands
        .spawn_bundle(root(&colors))
        .with_children(|parent| {
            parent.spawn_bundle(main_text(&colors, &textures, text));

            for (text, b) in buttons {
                parent
                    .spawn_bundle(button(&colors))
                    .with_children(|parent| {
                        match b {
                            MenuButton::Play => parent.spawn_bundle(button_icon(textures.play.clone())),
                            MenuButton::Quit => parent.spawn_bundle(button_icon(textures.exit.clone())),
                            MenuButton::MainMenu => parent.spawn_bundle(button_icon(textures.main.clone())),
                        };

                        parent.spawn_bundle(button_text(
                            &colors,
                            &textures,
                            text,
                        ));
                    })
                    .insert(b);
            }
        })
        .id();

    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    commands.insert_resource(MenuData {
        menu_entity,
        camera_entity,
    });
}

fn root(materials: &Res<MenuColors>) -> NodeBundle {
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

fn button(materials: &Res<MenuColors>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(380.0), Val::Px(65.0)),
            margin: Rect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: materials.button,
        ..default()
    }
}

fn button_text(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    label: &str,
) -> TextBundle {
    text_bundle(colors, textures, label, 40., None)
}

fn main_text(
    colors: &Res<MenuColors>,
    textures: &Res<MenuTextures>,
    label: &str,
) -> TextBundle {
    text_bundle(colors, textures, label, 80., Some(Style {
        margin: Rect::all(Val::Px(50.0)),
        ..default()
    }))
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

fn button_icon(
    icon: Handle<Image>,
) -> ImageBundle {
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

