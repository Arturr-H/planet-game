use bevy::prelude::*;

// list of different sound effects (wood, stone etc)
pub enum SoundGroup {
    Wood,
    Break,
    Power,
}


pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        //...
    }
}

impl AudioPlugin {
    //each sound effect will have a different sound file, and each sound effect can have multiple sound files
    pub fn load_audio(asset_server: &Res<AssetServer>, sound_group: SoundGroup) -> Handle<AudioSource> {
        match sound_group {
            SoundGroup::Wood => asset_server.load("audio/wood.wav"),
            SoundGroup::Break => asset_server.load("audio/break.wav"),
            SoundGroup::Power => asset_server.load("audio/power.wav"),
        }
    }
    
    
    
    
    
    pub fn play_audio(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        sound_group: SoundGroup,
        loop_audio: bool
    ) {
        let sound = Self::load_audio(asset_server, sound_group);
        if loop_audio {
            commands.spawn((AudioPlayer::new(sound), PlaybackSettings::LOOP));
        } else {
            commands.spawn((AudioPlayer::new(sound), PlaybackSettings::DESPAWN));
        }
    }
}

