/* Imports */
use std::hash::Hash;
use bevy::{prelude::*, utils::{HashMap, HashSet}};
use crate::{components::{cable::slot::CableSlot, tile::{empty::EmptyTile, Tile}}, systems::traits::{EnergyStorage, GenericTile}};
use super::Resources;

/// App state
#[derive(Resource)]
pub struct GameState {
    /// All tiles on the planet (id, tile)
    pub tiles: HashMap<usize, Tile>,
    tile_id: usize,

    pub resources: Resources,
    pub planet_entity: Option<Entity>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tiles: HashMap::new(),
            tile_id: 0,
            resources: Resources::default(),
            planet_entity: None,
        }
    }
}

impl GameState {
    fn tick(
        mut game_state: ResMut<Self>,
        time: Res<Time>,
        mut timer: Local<Option<Timer>>,
    ) -> () {
        let timer = timer.get_or_insert_with(||
            Timer::from_seconds(1.0, TimerMode::Repeating)
        );

        if timer.tick(time.delta()).just_finished() {
            let keys = game_state.tiles.keys().cloned().collect::<Vec<usize>>();
            for key in keys {
                let tile = game_state.tiles.get(&key).unwrap();
                if tile.can_distribute_energy() {
                    Tile::distribute_energy(
                        tile.energy_output(),
                        tile.tile_id,
                        &mut game_state
                    );
                }
            }
        }
    }

    /// Increments the tile_id and then returns it
    pub fn new_tile_id(&mut self) -> usize {
        let slot_id = self.tile_id;
        self.tile_id += 1;
        slot_id
    }

    /// If two tiles are connected via cables
    pub fn powergrid_tiles_are_connected(&self, a: usize, b: usize) -> bool {
        match self.tiles.get(&a) {
            Some(e) => e.powergrid_status().connected_tiles.contains(&b),
            None => false,
        }
    }
    pub fn powergrid_register_connection(&mut self, a: usize, b: usize) -> () {
        if let Some(e) = self.tiles.get_mut(&a) {
            e.powergrid_status_mut().connected_tiles.push(b);
        }
        if let Some(e) = self.tiles.get_mut(&b) {
            e.powergrid_status_mut().connected_tiles.push(a);
        }
    }

    /// Get planet entity or panic
    pub fn planet_entity(&self) -> Entity { self.planet_entity.unwrap() }
}

/// Game plugin
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .add_systems(Update, GameState::tick);
    }
}
