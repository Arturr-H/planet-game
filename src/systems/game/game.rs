/* Imports */
use bevy::prelude::*;

/// The state of the game. 
#[derive(Resource)]
pub struct GameState {
    /// What planet ID we're at. ????
    /// TODO: this is stupid why do we have this
    pub planet_id: usize,

    /// Seed of the game we're in
    pub game_seed: u64,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            planet_id: 0,
            game_seed: 0,
        }
    }
}

impl GameState {
    fn background_audio(
        mut _commands: Commands,
        _asset_server: Res<AssetServer>,
    ) {
        // let background_audio = asset_server.load("../assets/audio/forest.wav");

        // commands.spawn((
        //     AudioPlayer::new(background_audio),
        //     PlaybackSettings::LOOP,
        // ));
    }

    /// Returns a new planet ID
    pub fn new_planet_id(&mut self) -> usize {
        let id = self.planet_id;
        self.planet_id += 1;
        id
    }

    /// Sets the game seed
    pub fn set_game_seed(&mut self, game_seed: u64) -> () {
        self.game_seed = game_seed;
    }
}

/// Game plugin
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .add_systems(Startup, GameState::background_audio);
    }
}
