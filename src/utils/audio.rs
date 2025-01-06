use bevy::{audio::PlaybackMode, prelude::*};
use rand::{seq::SliceRandom, Rng};

// list of different sound effects (wood, stone etc)
pub mod game_sounds {
    pub mod tree {
        pub const DAMAGE: &'static [&str] = &[
            "audio/tree/damage_1.wav",
            "audio/tree/damage_2.wav",
            "audio/tree/damage_3.wav",
            "audio/tree/damage_4.wav",
            "audio/tree/damage_5.wav",
            "audio/tree/damage_6.wav",
            ];
        pub const FALL: &'static str = "audio/tree/fall.wav";
    }

    pub mod other {}
}

fn load_audio(asset_server: &Res<AssetServer>, sound_path: &'static str) -> Handle<AudioSource> {
    asset_server.load(sound_path)
}

pub fn play_audio(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sounds: &'static [&'static str],
    loop_audio: bool
) {
    if let Some(sound_path) = sounds.choose(&mut rand::thread_rng()) {
        let sound = load_audio(asset_server, sound_path);
        if loop_audio {
            commands.spawn((
                AudioPlayer::new(sound),
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    ..Default::default()
                }
            ));
        } else {
            commands.spawn((
                AudioPlayer::new(sound),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    speed: rand::thread_rng().gen_range(0.8..1.2),
                    ..Default::default()
                }
            ));
        }
    }
}