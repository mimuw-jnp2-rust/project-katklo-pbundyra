use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use rand::{thread_rng, Rng};

use crate::AppState;

pub struct GameAudioPlugin;

pub struct AudioHitEvent;
pub struct AudioCoffeeEvent;
pub struct AudioRustEvent;
pub struct AudioFastShootEvent;
pub struct AudioShootEvent;
pub struct AudioDeadPlayerEvent;

pub struct SimpleAudioEvent {
    pub audio_src: Handle<AudioSource>,
}

pub struct ComplexedAudioEvent {
    audio_assets: Vec<Handle<AudioSource>>,
}

pub struct AudioAssets {
    bg: Handle<AudioSource>,
    menu: Handle<AudioSource>,
    hits: Vec<Handle<AudioSource>>,
    death: Handle<AudioSource>,
    drinks: Vec<Handle<AudioSource>>,
    shoot: Handle<AudioSource>,
    fast_shoot: Handle<AudioSource>,
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
            // .add_system(play_hit_sfx)
            // .add_system(play_death_sfx)
            // .add_system(play_eat_sfx)
            // .add_system(play_lvlup_sfx)
            // .add_system(play_shoot_sfx)
            // .add_system(play_fast_shoot_sfx)
            .add_system(play_simple_audio)
            // .add_system(play_complexed_audio)
            .add_system(play_audio_for_event)
            .add_startup_system(play_menu_music)
            .add_event::<AudioRustEvent>()
            .add_event::<AudioCoffeeEvent>()
            .add_event::<AudioHitEvent>()
            .add_event::<AudioFastShootEvent>()
            .add_event::<AudioShootEvent>()
            .add_event::<AudioDeadPlayerEvent>()
            .add_event::<SimpleAudioEvent>()
            .add_event::<ComplexedAudioEvent>();
            .add_event::<AudioEvent>();
    }
}

pub fn play_simple_audio(audio: Res<Audio>, mut audio_event: EventReader<SimpleAudioEvent>) {
    audio_event.iter().for_each(|event| {
        audio.play(event.audio_src.clone());
    });
}

// pub fn play_complexed_audio(
//     audio: Res<Audio>,
//     mut audio_event: EventReader<ComplexedAudioEvent>,
// ){
//     audio_event.iter().for_each(|_| {
//         let mut rng = thread_rng();
//         audio.play(
//             audio_event.audio_assets[         rng.gen_range(0..audio_event.audio_assets.len())].clone());
//     }
// }

pub fn play_hit_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_event: EventReader<AudioHitEvent>,
) {
    audio_event.iter().for_each(|_| {
        let mut rng = thread_rng();
        match rng.gen_range(0..=2) {
            // 1/3 chance of playing each sound
            0 => audio.play(audio_state.hit1.clone()),
            1 => audio.play(audio_state.hit2.clone()),
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
        match rng.gen_range(0..=2) {
            // 1/3 chance of playing each sound
            0 => audio.play(audio_state.drink1.clone()),
            1 => audio.play(audio_state.drink2.clone()),
            _ => audio.play(audio_state.drink3.clone()),
        };
    });
}

pub fn play_audio_for_event(
    audio: Res<Audio>,
    audio_state: Res<AudioAssets>,
    mut audio_events: EventReader<AudioEvent>,
) {
    audio_events.iter().for_each(|event| {
        let sound = match event.audio_t {
            AudioType::Hit => {
                let sound_id = thread_rng().gen_range(0..audio_state.hits.len());
                audio_state.hits[sound_id].clone()
            }
            AudioType::Coffee => {
                let sound_id = thread_rng().gen_range(0..audio_state.drinks.len());
                audio_state.drinks[sound_id].clone()
            }
            AudioType::Rust => audio_state.lvlup.clone(),
            AudioType::FastShoot => audio_state.fast_shoot.clone(),
            AudioType::Shoot => audio_state.shoot.clone(),
            AudioType::DeadPlayer => audio_state.death.clone(),
        };

        audio.play(sound);
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
