/* Imports */
use std::hash::Hash;
use bevy::{prelude::*, utils::{HashMap, HashSet}};
use crate::{components::{slot::Slot, tile::{empty::EmptyTile, Tile}}, systems::traits::{EnergyStorage, GenericTile}, PLANET_SLOTS};
use super::Resources;

/// App state
#[derive(Resource)]
pub struct GameState {
    /// All slots on the planet (id, tile)
    pub slots: HashMap<usize, Tile>,
    pub slot_id: usize,

    pub resources: Resources
}

impl Default for GameState {
    fn default() -> Self {
        let mut slots = HashMap::new();
        for slot_id in 0..PLANET_SLOTS {
            slots.insert(slot_id, Tile::Empty(EmptyTile::new(slot_id)));
        }

        Self {
            slots,
            slot_id: PLANET_SLOTS,
            resources: Resources::default()
        }
    }
}

impl GameState {
    fn tick(
        mut game_state: ResMut<Self>,
        tile_q: Query<&Tile, With<Tile>>,
        time: Res<Time>,
        mut timer: Local<Option<Timer>>,
    ) -> () {
        let timer = timer.get_or_insert_with(||
            Timer::from_seconds(1.0, TimerMode::Repeating)
        );

        if timer.tick(time.delta()).just_finished() {
            for tile in tile_q.iter() {
                tile.distribute_energy(&mut game_state);
            }
        }
    }

    /// Increments the slot id and then returns it
    pub fn new_slot_id(&mut self) -> usize {
        let slot_id = self.slot_id;
        self.slot_id += 1;
        slot_id
    }

    /// If two slots are connected
    pub fn powergrid_tiles_are_connected(&self, a: usize, b: usize) -> bool {
        match self.slots.get(&a) {
            Some(e) => e.powergrid_status().connected_tiles.contains(&b),
            None => false,
        }
    }
    pub fn powergrid_register_connection(&mut self, a: usize, b: usize) -> () {
        if let Some(e) = self.slots.get_mut(&a) {
            e.powergrid_status_mut().connected_tiles.push(b);
        }
        if let Some(e) = self.slots.get_mut(&b) {
            e.powergrid_status_mut().connected_tiles.push(a);
        }
    }
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
