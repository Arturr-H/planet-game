/* Imports */
use bevy::prelude::*;
use crate::{components::{planet::Planet, poi::PointOfInterestType}, tile::{TileType, spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::game::PlanetResource};

#[enum_delegate::register]
#[allow(unused_variables)]
pub trait GenericTile {
    /// Spawn logic (bevy)
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        spawn_params: &mut TileSpawnEventParams,
        spawn_data: &TileSpawnEvent,
    ) -> Entity;

    /// What resources this tile costs
    fn cost(&self) -> Vec<(PlanetResource, usize)>;

    /// Returns the name of the object that will be displayed in game.
    fn display_name(&self) -> String;

    /// What will happen when this tile recieves
    /// energy (energy already added before this point)
    fn on_energy_recieved(&self, tile_id: usize, planet: &mut Planet) -> () {
        // Default is to do nothing
    }

    /// Tiles that produce energy should implement this, like
    /// solar panels, wind turbines, etc. (Energy per gametick)
    fn energy_output(&self) -> Option<f32> { None }

    /// Tiles that store energy should implement this, like
    /// batteries, etc.
    fn can_recieve_energy(&self) -> bool { false }

    /// How many tile slots this takes up
    fn width(&self) -> usize { 1 }

    /// What POI:s this tile interacts with
    fn interacts_with(&self) -> Vec<PointOfInterestType> { Vec::new() }

    /// What tiles this tile needs to "keep distance" from
    /// to avoid collisions looking ugly. Like wind turbines
    /// which would cause the rotors to overlap.
    /// 
    /// Returns Vec<(a, b)> where a is the minimum radius needed
    /// from tile type b to place this tile. Checking if tiles are
    /// being placed on one another is already implemented. 
    fn keep_distance_from(&self) -> Vec<(usize, TileType)> { Vec::new() }
}
