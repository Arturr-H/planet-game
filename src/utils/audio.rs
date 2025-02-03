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

    pub mod stone {
        pub const DAMAGE: &'static [&str] = &[
            "audio/stone/damage_1.wav",
            "audio/stone/damage_2.wav",
            "audio/stone/damage_3.wav",
            "audio/stone/damage_4.wav",
            "audio/stone/damage_5.wav",
        ];
    }

    pub mod other {}
}

#[derive(Resource)]
pub struct AudioManager {
    asset_server: AssetServer,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioManager>()
            .add_systems(Update, handle_audio_events)
            .add_event::<PlayAudioEvent>();
    }
}

impl FromWorld for AudioManager {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
        Self { asset_server }
    }
}

#[derive(Event)]
pub struct PlayAudioEvent {
    sounds: &'static [&'static str],
    loop_audio: bool,
}

fn handle_audio_events(
    mut commands: Commands,
    audio_manager: Res<AudioManager>,
    mut play_events: EventReader<PlayAudioEvent>,
) {
    for event in play_events.read() {
        if let Some(sound_path) = event.sounds.choose(&mut rand::thread_rng()) {
            let sound = audio_manager.asset_server.load(*sound_path);
            if event.loop_audio {
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
}

pub fn play_audio(
    sounds: &'static [&'static str],
    loop_audio: bool,
    event_writer: &mut EventWriter<PlayAudioEvent>,
) {
    event_writer.send(PlayAudioEvent { sounds, loop_audio });
}