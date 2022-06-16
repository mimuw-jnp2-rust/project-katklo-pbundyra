use bevy::prelude::*;

use crate::menu::MenuButton;

pub struct MenuData {
    menu_entity: Entity,
    camera_entity: Entity,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

pub struct MenuMaterials {
    root: UiColor,
    border: UiColor,
    button: UiColor,
    menu: UiColor,
    button_text: Color,
    button_hovered: UiColor,
    button_pressed: UiColor,
}

impl Default for MenuMaterials {
    fn default() -> Self {
        MenuMaterials {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_text: Color::WHITE,
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
        }
    }
}


pub fn button_system(
    mut buttons: Query<(&Interaction, &mut UiColor, Changed<Interaction>), With<Button>>,
    materials: Res<MenuMaterials>,
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
             asset_server: Res<AssetServer>,
             materials: Res<MenuMaterials>,
             buttons: Vec<(&'static str, MenuButton)>) {
    let menu_entity = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            for (text, b) in buttons {
                                parent
                                    .spawn_bundle(button(&materials))
                                    .with_children(|parent| {
                                        parent.spawn_bundle(button_text(
                                            &asset_server,
                                            &materials,
                                            text,
                                        ));
                                    })
                                    .insert(b);
                            }
                        });
                });
        })
        .id();

    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    commands.insert_resource(MenuData {
        menu_entity,
        camera_entity,
    });
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: materials.root,
        ..default()
    }
}

fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
            ..default()
        },
        color: materials.border,
        ..default()
    }
}

fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(5.0)),
            ..default()
        },
        color: materials.menu,
        ..default()
    }
}

fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: materials.button,
        ..default()
    }
}

fn button_text(
    asset_server: &Res<AssetServer>,
    materials: &Res<MenuMaterials>,
    label: &str,
) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-LightItalic.ttf"),
                font_size: 30.0,
                color: materials.button_text,
            },
            Default::default(),
        ),
        ..default()
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_entity).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}