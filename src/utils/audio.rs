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

    pub const PLACE: &'static [&str] = &["audio/place.wav"];
    pub const DRILL: &'static [&str] = &["audio/drill.wav"];

    pub mod other {}
}

#[derive(Resource)]
pub struct AudioManager {
    asset_server: AssetServer,
}

impl FromWorld for AudioManager {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
        Self { asset_server }
    }
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioManager>()
            .add_systems(Update, handle_audio_events)
            .add_event::<PlayAudioEvent>();
    }
}


#[derive(Event)]
pub struct PlayAudioEvent {
    sounds: &'static [&'static str],
    playback_settings: PlaybackSettings,
    position: Option<Vec3>,
}

fn handle_audio_events(
    mut commands: Commands,
    audio_manager: Res<AudioManager>,
    mut play_events: EventReader<PlayAudioEvent>,
) {
    for event in play_events.read() {
        if let Some(sound_path) = event.sounds.choose(&mut rand::thread_rng()) {
            let sound = audio_manager.asset_server.load(*sound_path);

            commands.spawn((
                AudioPlayer::new(sound),
                event.playback_settings.with_speed(rand::thread_rng().gen_range(0.8..1.2)),
                Transform::from_translation(event.position.unwrap_or(Vec3::ZERO)),
            ));
        }
    }
}

pub fn play_audio(
    sounds: &'static [&'static str],
    playback_settings: PlaybackSettings,
    position: Option<Vec3>,
    event_writer: &mut EventWriter<PlayAudioEvent>,
) {
    event_writer.send(PlayAudioEvent { sounds, playback_settings, position });
}