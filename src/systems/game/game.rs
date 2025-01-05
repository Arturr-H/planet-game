/* Imports */
use std::hash::Hash;
use bevy::{prelude::*, utils::{HashMap, HashSet}};
use crate::{components::{cable::slot::CableSlot, planet::Planet, tile::{types::empty::EmptyTile, Tile}}, systems::traits::{EnergyStorage, GenericTile}};

/// The state of the game. 
#[derive(Resource)]
pub struct GameState {
    /// What planet ID we're at.
    planet_id: usize,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            planet_id: 0,
        }
    }
}

impl GameState {
    fn background_audio(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let background_audio = asset_server.load("../assets/audio/forest.wav");

        commands.spawn((
            AudioPlayer::new(background_audio),
            PlaybackSettings::LOOP,
        ));
    }

    /// Returns a new planet ID
    pub fn new_planet_id(&mut self) -> usize {
        let id = self.planet_id;
        self.planet_id += 1;
        id
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
