use bevy::app::AppExit;
use bevy::prelude::*;

use crate::menu::structs::{InputText, MenuButton, MenuColors, SelectedOption};
use crate::menu::utils::{start_game_for_level, start_new_game};
use crate::{AppState, Level, Random};

pub fn button_press_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    buttons: Query<(&Interaction, &MenuButton, Entity, Changed<Interaction>), With<Button>>,
    mut exit: EventWriter<AppExit>,
    mut rng: ResMut<Random>,
    mut level: ResMut<Level>,
) {
    for (interaction, button, entity, _) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::NewGame => start_new_game(&mut state, &mut rng, &mut level),
                MenuButton::Quit => exit.send(AppExit),
                MenuButton::MainMenu => state
                    .set(AppState::MainMenu)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::SeedGenerate => rng.new_random_seed(),
                MenuButton::InputButton => {
                    *rng = Random::new();
                    rng.can_change = true;
                    commands.entity(entity).insert(SelectedOption);
                }
                MenuButton::NextLevel => {
                    level.increase_level();
                    start_game_for_level(&mut state, &mut rng, &mut level);
                }
                MenuButton::RestartLevel => start_game_for_level(&mut state, &mut rng, &mut level),
                MenuButton::RestartGame => start_new_game(&mut state, &mut rng, &mut level),
            };
        }
    }
}

pub fn button_system(
    materials: Res<MenuColors>,
    mut buttons: Query<(&Interaction, &mut UiColor, Option<&SelectedOption>), With<Button>>,
) {
    for (interaction, mut color, selected) in buttons.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => materials.button_pressed,
            (Interaction::Hovered, Some(_)) => materials.button_selected,
            (Interaction::Hovered, None) => materials.button_hovered,
            (Interaction::None, Some(_)) => materials.button_selected,
            (Interaction::None, None) => materials.normal_button,
        }
    }
}

pub fn input_button_system(
    mut commands: Commands,
    mut random: ResMut<Random>,
    materials: Res<MenuColors>,
    mut buttons: Query<(&Interaction, &MenuButton, Changed<Interaction>), With<Button>>,
    mut selected_query: Query<(Entity, &mut UiColor), With<SelectedOption>>,
) {
    for (interaction, button, _) in buttons.iter_mut() {
        if let MenuButton::InputButton = button {
            continue;
        } else if *interaction == Interaction::Clicked {
            if let Ok((selected, mut selected_color)) = selected_query.get_single_mut() {
                random.can_change = false;
                *selected_color = materials.normal_button;
                commands.entity(selected).remove::<SelectedOption>();
            }
        }
    }
}

pub fn read_input_system(
    mut random: ResMut<Random>,
    keys: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
) {
    for ev in char_evr.iter() {
        if ev.char.is_alphanumeric() {
            random.add_char(ev.char);
        }
    }

    if keys.just_pressed(KeyCode::Back) {
        random.delete_last();
    }
}

pub fn text_update_system(random: Res<Random>, mut query: Query<&mut Text, With<InputText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = random.seed.clone();
    }
}
