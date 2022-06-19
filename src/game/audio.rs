use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use rand::{thread_rng, Rng};

use crate::game::living_being::{LivingBeingDeathEvent, LivingBeingHitEvent};
use crate::game::player::DeadPlayerEvent;
use crate::game::powerups::CoffeeEvent;
use crate::game::{FastShootEvent, RustEvent, ShootEvent};
use crate::AppState;

pub struct GameAudioPlugin;

pub struct AudioHitEvent;
pub struct AudioCoffeeEvent;
pub struct AudioRustEvent;
pub struct AudioFastShootEvent;
pub struct AudioShootEvent;
pub struct AudioDeadPlayerEvent;

pub struct AudioAssets {
    bg: Handle<AudioSource>,
    menu: Handle<AudioSource>,
    hit1: Handle<AudioSource>,
    hit2: Handle<AudioSource>,
    hit3: Handle<AudioSource>,
    death: Handle<AudioSource>,
    drink1: Handle<AudioSource>,
    drink2: Handle<AudioSource>,
    drink3: Handle<AudioSource>,
    shoot: Handle<AudioSource>,
    fast_shoot: Handle<AudioSource>,
    lvlup: Handle<AudioSource>,
    bg_channel: AudioChannel,
    menu_channel: AudioChannel,
}

const SFX_VOLUME: f32 = 0.35;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(play_bg_music))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(play_menu_music))
            .add_system(play_hit_sfx)
            .add_system(play_death_sfx)
            .add_system(play_eat_sfx)
            .add_system(play_lvlup_sfx)
            .add_system(play_shoot_sfx)
            .add_system(play_fast_shoot_sfx)
            .add_startup_system(play_menu_music)
            .add_event::<AudioRustEvent>()
            .add_event::<AudioCoffeeEvent>()
            .add_event::<AudioHitEvent>()
            .add_event::<AudioFastShootEvent>()
            .add_event::<AudioShootEvent>()
            .add_event::<AudioDeadPlayerEvent>();
    }
}

pub fn play_hit_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioHitEvent>,
) {
    audio_event.iter().for_each(|_| {
        let mut rng = thread_rng();
        match rng.gen_range(0..100) {
            // 1/3 chance of playing each sound
            0..=33 => audio.play(audio_state.hit1.clone()),
            34..=67 => audio.play(audio_state.hit2.clone()),
            _ => audio.play(audio_state.hit3.clone()),
        };
    });
}

pub fn play_eat_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioCoffeeEvent>,
) {
    audio_event.iter().for_each(|_| {
        let mut rng = thread_rng();
        match rng.gen_range(0..100) {
            // 1/3 chance of playing each sound
            0..=33 => audio.play(audio_state.drink1.clone()),
            34..=67 => audio.play(audio_state.drink2.clone()),
            _ => audio.play(audio_state.drink3.clone()),
        };
    });
}

// TODO moze generyk
pub fn play_lvlup_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioRustEvent>,
) {
    audio_event.iter().for_each(|_| {
        audio.play(audio_state.lvlup.clone());
    });
}

pub fn play_shoot_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioShootEvent>,
) {
    audio_event.iter().for_each(|_| {
        audio.play(audio_state.shoot.clone());
    });
}

pub fn play_fast_shoot_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioFastShootEvent>,
) {
    audio_event.iter().for_each(|_| {
        audio.play(audio_state.fast_shoot.clone());
    });
}

pub fn play_death_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut death_events: EventReader<AudioDeadPlayerEvent>,
) {
    for _ in death_events.iter() {
        audio.play(audio_state.death.clone());
    }
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
        drink1: assets.load("audio/drink1.ogg"),
        drink2: assets.load("audio/drink2.ogg"),
        drink3: assets.load("audio/drink3.ogg"),
        shoot: assets.load("audio/shoot.ogg"),
        fast_shoot: assets.load("audio/fast_shoot.ogg"),
        lvlup: assets.load("audio/levelup.ogg"),
        bg_channel: AudioChannel::new("bg".to_string()),
        menu_channel: AudioChannel::new("menu".to_string()),
    });
}
