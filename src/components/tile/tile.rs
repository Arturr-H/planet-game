/* Imports */
use std::mem::discriminant;
use bevy::{prelude::*, sprite::Material2dPlugin, utils::HashMap};
use crate::{components::{planet::Planet, poi::PointOfInterestType}, systems::{game::PlanetResource, traits::{EnergyStorage, GenericTile, PowergridStatus}}};
use super::{material::TileMaterialOutline, spawn::{TileSpawnEvent, TileSpawnEventParams, TileSpawnPlugin}, types::{battery::Battery, debug::DebugTile, drill::Drill, empty::EmptyTile, landed_rocket::LandedRocket, power_pole::PowerPole, solar_panel::SolarPanel, wind_turbine::WindTurbine}};

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

    /// What level of upgrade the tile is on. 0 = base level,
    /// and with this number increasing, the tile will improve
    /// and change appearance.
    pub tile_level: usize,

    /// Aka planet_position_index. The index of the tile
    /// in the planet's tile grid.
    pub tile_id: usize,
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
    pub fn new(tile_id: usize, tile_type: TileType, tile_level: usize, entity: Entity) -> Self {
        Self {
            tile_type,
            powergrid_status: PowergridStatus::default(),
            tile_id,
            tile_level,
            entity
        }
    }

    /// Distribute energy across cables from this tile.
    /// Only runs from generators.
    /// 
    /// `energy_to_add` is a HashMap containing the tile_id
    /// and the amount of energy to add to that tile. We need
    /// to have an external variable beacuse this function is 
    /// run for each individual tile, and we only want to run
    /// the `.on_energy_recieved` function once for each tile.
    pub fn distribute_energy_from(
        tile_id: usize,
        energy_output: f32,
        energy_to_add: &mut HashMap<usize, f32>,
        planet: &Planet
    ) -> () {
        // HashMap<tile_id, will_recieve_energy>
        let mut visited: HashMap<usize, bool> = HashMap::new();
        let mut recievers = 0;

        Self::search_tile(
            &mut recievers,
            planet,
            tile_id,
            &mut visited
        );

        let energy_per_reciever = energy_output / recievers.max(1) as f32;
        for (tile_id, will_recieve_energy) in visited {
            if will_recieve_energy {
                match energy_to_add.get_mut(&tile_id) {
                    Some(e) => *e += energy_per_reciever,
                    None => {
                        energy_to_add.insert(tile_id, energy_per_reciever);
                    }
                }
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
    pub fn energy_output(&self) -> Option<f32> {
        self.tile_type.energy_output(self)
    }
    pub fn can_recieve_energy(&self) -> bool {
        self.tile_type.can_recieve_energy()
    }

    /// Adds energy to all tiles implementing `EnergyStorage`
    pub fn add_energy(planet: &mut Planet, tile_id: usize, energy: f32) -> () {
        // Add energy to the tile
        match planet.tiles.get_mut(&tile_id) {
            Some(e) => {
                let stored = e.powergrid_status.energy_stored;
                e.powergrid_status.energy_stored = (stored + energy).min(e.tile_type.energy_capacity(&e))
            },
            None => (),
        };
    }

    /// Generates a spread of indexes based on a given width and starting index, with wrapping around at boundaries.
    /// # Returns
    /// 
    /// A `Vec<usize>` containing the calculated indexes in the spread. The spread grows primarily to the right if
    /// `width` is even, or symmetrically left and right if `width` is odd. If the spread exceeds the boundaries of
    /// the available indexes, it wraps around to the other end.
    pub fn get_tile_spread(width: usize, index: usize, boundary: usize) -> Vec<usize> {
        assert!(width > 0, "Width must be at least 1.");
        assert!(index < boundary, "Index must be within the range of available indexes.");
    
        let mut spread = Vec::with_capacity(width);
        let half_width = width / 2;
    
        for i in 0..width {
            let offset = if width % 2 == 1 {
                // If width is odd, grow left and right
                i as isize - half_width as isize
            }else {
                // If width is even, grow primarily to the right
                i as isize - (half_width as isize - 1)
            };
    
            let wrapped_index = ((index as isize + offset).rem_euclid(boundary as isize)) as usize;
            spread.push(wrapped_index);
        }
    
        spread.sort(); // Ensures consistent ordering of results
        spread
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

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                // DrillPlugin,
                Material2dPlugin::<TileMaterialOutline>::default(),
                TileSpawnPlugin
            ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile_spread() {
        let spread = Tile::get_tile_spread(2, 0, 5);
        assert_eq!(spread, vec![0, 1]);

        let spread = Tile::get_tile_spread(3, 0, 5);
        assert_eq!(spread, vec![0, 1, 4]);

        let spread = Tile::get_tile_spread(3, 5, 20);
        assert_eq!(spread, vec![4, 5, 6]);

        let spread = Tile::get_tile_spread(7, 5, 20);
        assert_eq!(spread, vec![2, 3, 4, 5, 6, 7, 8]);
        let spread = Tile::get_tile_spread(8, 5, 20);
        assert_eq!(spread, vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
