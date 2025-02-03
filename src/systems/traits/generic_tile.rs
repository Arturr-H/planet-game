/* Imports */
use bevy::prelude::*;
use crate::{
    components::{planet::Planet, poi::PointOfInterestType},
    systems::game::PlanetResource,
    tile::{spawn::{TileSpawnEvent, TileSpawnEventParams}, Tile, TileType},
    utils::audio::{PlayAudioEvent, game_sounds},
};

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

    /// What will happen every tick, before this tile recieves
    /// energy.
    fn on_tick(&self, tile_id: usize, planet: &mut Planet, audio_events: &mut EventWriter<PlayAudioEvent>) -> () {
        // Default is to do nothing
    }

    /// Tiles that produce energy should implement this, like
    /// solar panels, wind turbines, etc. (Energy per gametick)
    fn energy_output(&self, tile: &Tile) -> Option<f32> { None }

    /// How much energy this tile can store, as a maximum
    fn energy_capacity(&self, tile: &Tile) -> f32 { 50.0 }

    /// Tiles that store energy should implement this, like
    /// batteries, etc.
    fn can_recieve_energy(&self) -> bool { false }

    /// How many tile slots this takes up
    fn width(&self) -> usize { 1 }

    /// What POI:s this tile interacts with
    fn interacts_with(&self) -> Vec<PointOfInterestType> { Vec::new() }

    /// What upgrades this tile has.
    /// 
    /// The outer vector is the upgrade level, the inner vector
    /// is the cost of the upgrade. So `.upgrades()[0]` is the cost
    /// of upgrading to level 1. The cost of actually building the
    /// tile is not included in this vector, it can be found in [`Self::cost`]
    fn upgrades(&self) -> Vec<Vec<(PlanetResource, usize)>> { Vec::new() }

    /// Tiles that can't be removed by the player, like the rocketship
    fn indestructible(&self) -> bool { false }

    /// What tiles this tile needs to "keep distance" from
    /// to avoid collisions looking ugly. Like wind turbines
    /// which would cause the rotors to overlap.
    /// 
    /// Returns Vec<(a, b)> where a is the minimum radius needed
    /// from tile type b to place this tile. Checking if tiles are
    /// being placed on one another is already implemented. 
    fn keep_distance_from(&self) -> Vec<(usize, TileType)> { Vec::new() }
}
