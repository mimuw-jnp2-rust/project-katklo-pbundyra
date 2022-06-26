use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use rand::{thread_rng, Rng};

use crate::AppState;

pub struct GameAudioPlugin;

pub struct SimpleAudioEvent {
    pub audio_src: Handle<AudioSource>,
}

pub struct ComplexAudioEvent {
    pub audio_src: Vec<Handle<AudioSource>>,
}

pub struct AudioAssets {
    pub bg: Handle<AudioSource>,
    pub menu: Handle<AudioSource>,
    pub hits: Vec<Handle<AudioSource>>,
    pub death: Handle<AudioSource>,
    pub drinks: Vec<Handle<AudioSource>>,
    pub shoot: Handle<AudioSource>,
    pub fast_shoot: Handle<AudioSource>,
    pub lvlup: Handle<AudioSource>,
    pub bg_channel: AudioChannel,
    pub menu_channel: AudioChannel,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(play_bg_music))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(play_menu_music))
            .add_system(play_simple_audio)
            .add_system(play_complexed_audio)
            .add_startup_system(play_menu_music)
            .add_event::<SimpleAudioEvent>()
            .add_event::<ComplexAudioEvent>();
    }
}

pub fn play_simple_audio(audio: Res<Audio>, mut audio_event: EventReader<SimpleAudioEvent>) {
    audio_event.iter().for_each(|event| {
        audio.play(event.audio_src.clone());
    });
}

pub fn play_complexed_audio(
    audio: Res<Audio>,
    mut audio_event: EventReader<ComplexAudioEvent>,
    audio_assets: Res<AudioAssets>,
) {
    audio_event.iter().for_each(|event| {
        let sound_id = thread_rng().gen_range(0..audio_assets.hits.len());
        audio.play(event.audio_src[sound_id].clone());
    });
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
        hits: vec![
            assets.load("audio/hit1.ogg"),
            assets.load("audio/hit2.ogg"),
            assets.load("audio/hit3.ogg"),
        ],
        death: assets.load("audio/death.ogg"),
        drinks: vec![
            assets.load("audio/drink1.ogg"),
            assets.load("audio/drink2.ogg"),
            assets.load("audio/drink3.ogg"),
        ],
        shoot: assets.load("audio/shoot.ogg"),
        fast_shoot: assets.load("audio/fast_shoot.ogg"),
        lvlup: assets.load("audio/levelup.ogg"),
        bg_channel: AudioChannel::new("bg".to_string()),
        menu_channel: AudioChannel::new("menu".to_string()),
    });
}
