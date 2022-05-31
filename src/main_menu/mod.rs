use bevy::{app::AppExit, prelude::*};

use super::AppState;

pub struct MainMenuPlugin;

struct MainMenuData {
    menu_entity: Entity,
    camera_entity: Entity,
}

struct MenuMaterials {
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

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMaterials>()
            .add_system(button_press_system)
            .add_system(button_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_main_menu));
    }
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
            };
        }
    }
}

fn button_system(
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: Res<MenuMaterials>) {
    let menu_entity = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "New Game",
                                    ));
                                })
                                .insert(MenuButton::Play);
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "Quit",
                                    ));
                                })
                                .insert(MenuButton::Quit);
                        });
                });
        })
        .id();

    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    commands.insert_resource(MainMenuData {
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

fn cleanup_main_menu(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.menu_entity).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}
