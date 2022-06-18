use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};

use crate::{AppState};
use crate::game::living_being::LivingBeingHitEvent;

pub struct GameAudioPlugin;

pub struct AudioAssets {
    bg: Handle<AudioSource>,
    menu: Handle<AudioSource>,
    hit: Handle<AudioSource>,
    bg_channel: AudioChannel,
    menu_channel: AudioChannel,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(play_bg_music))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(play_bg_music))
            .add_system(play_hit_sfx)
            .add_startup_system(play_menu_music);
    }
}

fn play_hit_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut hit_events: EventReader<LivingBeingHitEvent>,
) {
    for _ in hit_events.iter() {
        audio.play(audio_state.hit.clone());
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

fn load_audio(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        bg: assets.load("audio/background.ogg"),
        menu: assets.load("audio/menu.ogg"),
        hit: assets.load("audio/hit.ogg"),
        bg_channel: AudioChannel::new("bg".to_string()),
        menu_channel: AudioChannel::new("menu".to_string()),
    });
}