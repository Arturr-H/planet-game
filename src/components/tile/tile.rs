/* Imports */
use std::mem::discriminant;
use bevy::{prelude::*, utils::HashMap};
use crate::{components::{planet::Planet, poi::PointOfInterestType}, systems::{game::PlanetResource, traits::{EnergyStorage, GenericTile, PowergridStatus}}};
use super::{spawn::{TileSpawnEvent, TileSpawnEventParams, TileSpawnPlugin}, types::{battery::Battery, debug::DebugTile, drill::Drill, empty::EmptyTile, landed_rocket::LandedRocket, power_pole::PowerPole, solar_panel::SolarPanel, wind_turbine::WindTurbine}};

/* Constants */
pub const TILE_SIZE: f32 = 20.0;

/// A tile is something that can be placed on
/// a planet. Can contain e.g solar panels,
/// power poles etc.
#[derive(Debug, Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub powergrid_status: PowergridStatus,
    pub entity: Entity,

    /// Aka planet_position_index. The index of the tile
    /// in the planet's tile grid.
    pub tile_identifier: usize,
}

/// Something that can be placed in a slot
#[enum_delegate::implement(GenericTile)]
#[derive(Component, Clone, Debug)]
pub enum TileType {
    Empty(EmptyTile),
    Drill(Drill),
    SolarPanel(SolarPanel),
    DebugTile(DebugTile),
    Battery(Battery),
    PowerPole(PowerPole),
    WindTurbine(WindTurbine),
    LandedRocket(LandedRocket),
}

// We only want to compare the type of Tile, the content
// of each enum variant is a ZST and doesn't need to be compared.
impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Tile {
    /// Creates a new tile
    pub fn new(tile_identifier: usize, tile_type: TileType, entity: Entity) -> Self {
        Self {
            tile_type,
            powergrid_status: PowergridStatus::default(),
            tile_identifier,
            entity
        }
    }

    /// Distribute energy across cables from this tile.
    /// Only runs from generators.
    pub fn distribute_energy(energy_output: f32, cable_slot_id: usize, planet: &mut Planet) -> () {
        // HashMap<tile_id, will_recieve_energy>
        let mut visited: HashMap<usize, bool> = HashMap::new();
        let mut recievers = 0;

        Self::search_tile(
            &mut recievers,
            planet,
            cable_slot_id,
            &mut visited
        );

        let energy_per_reciever = energy_output / recievers.max(1) as f32;
        for (tile_id, will_recieve_energy) in visited {
            if will_recieve_energy {
                Self::add_energy(planet, tile_id, energy_per_reciever);
            }
        }
    }
    fn search_tile(
        recievers: &mut usize,
        planet: &Planet,
        tile_id: usize,
        visited: &mut HashMap<usize, bool>,
    ) -> () {
        let Some(tile) = planet.tiles.get(&tile_id) else { return };
        visited.insert(tile_id, tile.can_recieve_energy());
        if tile.can_recieve_energy() {
            *recievers += 1;
        }
        
        for tile_id in &tile.powergrid_status().connected_tiles {
            if !visited.contains_key(tile_id) {
                Self::search_tile(recievers, planet, *tile_id, visited);
            }
        }
    }
    pub fn energy_output(&self) -> f32 {
        use TileType::*;

        match &self.tile_type {
            SolarPanel(_) => 1.0,
            WindTurbine(_) => 5.0,
            DebugTile(_) | Empty(_) => 0.0,
            PowerPole(_) => 0.0,
            Drill(_) => 0.0,
            Battery(_) => 0.0,
            LandedRocket(_) => 0.0,
        }
    }

    /// Adds energy to all tiles implementing `EnergyStorage`
    pub fn add_energy(planet: &mut Planet, tile_id: usize, energy: f32) -> () {
        // Add energy to the tile
        match planet.tiles.get_mut(&tile_id) {
            Some(e) => e.powergrid_status.energy_stored += energy,
            None => (),
        };
        
        let tile_type = planet.tiles[&tile_id].tile_type.clone();
        tile_type.on_energy_recieved(tile_id, planet);
    }
    pub fn can_recieve_energy(&self) -> bool {
        use TileType::*;

        match self.tile_type {
            DebugTile(_) | Drill(_) | Battery(_) => true,
            SolarPanel(_) | PowerPole(_) | Empty(_) | WindTurbine(_) | LandedRocket(_) => false,
        }
    }
    pub fn can_distribute_energy(&self) -> bool {
        use TileType::*;

        match self.tile_type {
            SolarPanel(_) | WindTurbine(_) => true,
            DebugTile(_) | PowerPole(_) | Empty(_) | Drill(_) | Battery(_) | LandedRocket(_) => false,
        }
    }

    pub fn tile_identifier(&self) -> usize { self.tile_identifier }
    pub fn powergrid_status(&self) -> &PowergridStatus { &self.powergrid_status }
    pub fn powergrid_status_mut(&mut self) -> &mut PowergridStatus { &mut self.powergrid_status }
}

// Most tiles should be able to store energy
impl EnergyStorage for Tile {
    fn stored(&self) -> f32 { self.powergrid_status.energy_stored }
    fn add_energy(&mut self, amount: f32) { self.powergrid_status.energy_stored += amount; }
}

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                // DrillPlugin,
                TileSpawnPlugin
            );
    }
}
