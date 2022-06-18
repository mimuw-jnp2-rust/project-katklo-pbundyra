use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use rand::{Rng, thread_rng};

use crate::{AppState};
use crate::game::living_being::{LivingBeingDeathEvent, LivingBeingHitEvent};
use crate::game::player::{DeadPlayerEvent, Player};
use crate::game::powerups::CoffeeEvent;

pub struct GameAudioPlugin;

pub struct AudioAssets {
    bg: Handle<AudioSource>,
    menu: Handle<AudioSource>,
    hit1: Handle<AudioSource>,
    hit2: Handle<AudioSource>,
    hit3: Handle<AudioSource>,
    death: Handle<AudioSource>,
    eat1: Handle<AudioSource>,
    eat2: Handle<AudioSource>,
    eat3: Handle<AudioSource>,
    lvlup: Handle<AudioSource>,
    bg_channel: AudioChannel,
    menu_channel: AudioChannel,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(play_bg_music))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(play_menu_music))
            .add_system(play_hit_sfx)
            .add_system(play_death_sfx)
            .add_system(play_eat_sfx)
            // .add_system(play_lvlup_sfx)
            .add_startup_system(play_menu_music);
    }
}

pub fn play_hit_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut hit_events: EventReader<LivingBeingHitEvent>,
) {
    for _ in hit_events.iter() {
        audio.set_volume(0.35);
        let mut rng = thread_rng();
        match rng.gen_range(0..100) {
            0..=33 => audio.play(audio_state.hit1.clone()),
            34..=67 => audio.play(audio_state.hit2.clone()),
            _ => audio.play(audio_state.hit3.clone()),
        };
    }
}

pub fn play_eat_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut coffee_events: EventReader<CoffeeEvent>,
) {
    for _ in coffee_events.iter() {
        audio.set_volume(0.35);
        let mut rng = thread_rng();
        match rng.gen_range(0..100) {
            0..=33 => audio.play(audio_state.eat1.clone()),
            34..=67 => audio.play(audio_state.eat2.clone()),
            _ => audio.play(audio_state.eat3.clone()),
        };
    }
}


pub fn play_lvlup_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
) {
    audio.play(audio_state.lvlup.clone());
}

pub fn play_death_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut death_events: EventReader<DeadPlayerEvent>,
    mut state: ResMut<State<AppState>>,
) {
    for _ in death_events.iter() {
        audio.play(audio_state.death.clone());
        state.set(AppState::DeathMenu).expect("Could not set state to DeathMenu");
    }
    // death_events.iter().for_each(|_| {
    //     audio.set_volume(0.35);
    //     audio.play(audio_state.death.clone());
    //     state
    //         .set(AppState::DeathMenu)
    //         .expect("Couldn't switch state to InGame");
    // });
}


fn play_bg_music(audio: Res<Audio>, audio_state: Res<AudioAssets>) {
    audio.stop_channel(&audio_state.menu_channel);
    audio.play_looped_in_channel(audio_state.bg.clone(), &audio_state.bg_channel);
}

fn play_menu_music(audio: Res<Audio>, audio_state: Res<AudioAssets>) {
    audio.stop_channel(&audio_state.bg_channel);
    audio.play_looped_in_channel(audio_state.menu.clone(), &audio_state.menu_channel);
}

fn load_audio(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        bg: assets.load("audio/background.ogg"),
        menu: assets.load("audio/menu.ogg"),
        hit1: assets.load("audio/hit1.ogg"),
        hit2: assets.load("audio/hit2.ogg"),
        hit3: assets.load("audio/hit3.ogg"),
        death: assets.load("audio/death.ogg"),
        eat1: assets.load("audio/eat1.ogg"),
        eat2: assets.load("audio/eat2.ogg"),
        eat3: assets.load("audio/eat3.ogg"),
        lvlup: assets.load("audio/levelup.ogg"),
        bg_channel: AudioChannel::new("bg".to_string()),
        menu_channel: AudioChannel::new("menu".to_string()),
    });
}