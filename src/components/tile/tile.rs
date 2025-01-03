/* Imports */
use bevy::{prelude::*, utils::HashMap};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::cable::slot::SLOT_SIZE, systems::{game::GameState, traits::{EnergyStorage, GenericTile, PowergridStatus}}, utils::color::hex};
use super::{debug::DebugTile, empty::EmptyTile, power_pole::PowerPole, solar_panel::SolarPanel};

/// A tile is something that can be placed on
/// a planet. Can contain e.g solar panels,
/// power poles etc.
#[derive(Debug, Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub tile_id: usize,
    pub powergrid_status: PowergridStatus,
}

/// Something that can be placed in a slot
#[enum_delegate::implement(GenericTile)]
#[derive(Component, Clone, Debug)]
pub enum TileType {
    Empty(EmptyTile),
    SolarPanel(SolarPanel),
    DebugTile(DebugTile),
    PowerPole(PowerPole)
}

impl Tile {
    /// Creates a new tile
    pub fn new(tile_id: usize, tile_type: TileType) -> Self {
        Self {
            tile_type,
            tile_id,
            powergrid_status: PowergridStatus::default()
        }
    }

    /// Distribute energy across cables from this tile.
    /// Only runs from generators.
    pub fn distribute_energy(energy_output: f32, cable_slot_id: usize, game_state: &mut GameState) -> () {
        // HashMap<tile_id, will_recieve_energy>
        let mut visited: HashMap<usize, bool> = HashMap::new();
        let mut recievers = 0;

        Self::search_tile(
            &mut recievers,
            game_state,
            cable_slot_id,
            &mut visited
        );

        let energy_per_reciever = energy_output / recievers.max(1) as f32;
        for (tile_id, will_recieve_energy) in visited {
            if will_recieve_energy {
                match game_state.tiles.get_mut(&tile_id) {
                    Some(e) => e.add_energy(energy_per_reciever),
                    None => ()
                };
            }
        }
    }
    fn search_tile(
        recievers: &mut usize,
        game_state: &GameState,
        tile_id: usize,
        visited: &mut HashMap<usize, bool>,
    ) -> () {
        let Some(tile) = game_state.tiles.get(&tile_id) else { return };
        visited.insert(tile_id, tile.can_store_energy());
        if tile.can_store_energy() {
            *recievers += 1;
        }
        
        for tile_id in &tile.powergrid_status().connected_tiles {
            if !visited.contains_key(tile_id) {
                Self::search_tile(recievers, game_state, *tile_id, visited);
            }
        }
    }
    pub fn energy_output(&self) -> f32 {
        use TileType::*;

        match &self.tile_type {
            SolarPanel(_) => 1.0,
            DebugTile(_) | Empty(_) => 0.0,
            PowerPole(_) => 0.0,
        }
    }

    /// Adds energy to all tiles implementing `EnergyStorage`
    pub fn add_energy(&mut self, energy: f32) -> () {
        use TileType::*;

        println!("{:?} recieved energy: {}", self.tile_type, energy);
        match &mut self.tile_type {
            DebugTile(_) => self.powergrid_status.energy_stored += energy,
            SolarPanel(_) | Empty(_) => (),
            PowerPole(_) => (),
        }
    }
    pub fn can_store_energy(&self) -> bool {
        use TileType::*;

        match self.tile_type {
            DebugTile(_) => true,
            SolarPanel(_) | PowerPole(_) | Empty(_) => false,
        }
    }
    pub fn can_distribute_energy(&self) -> bool {
        use TileType::*;

        match self.tile_type {
            SolarPanel(_) => true,
            DebugTile(_) | PowerPole(_) | Empty(_) => false,
        }
    }

    pub fn tile_id(&self) -> usize { self.tile_id }
    pub fn powergrid_status(&self) -> &PowergridStatus { &self.powergrid_status }
    pub fn powergrid_status_mut(&mut self) -> &mut PowergridStatus { &mut self.powergrid_status }
}

// Most tiles should be able to store energy
impl EnergyStorage for Tile {
    fn stored(&self) -> f32 { self.powergrid_status.energy_stored }
    fn add_energy(&mut self, amount: f32) { self.powergrid_status.energy_stored += amount; }
}
