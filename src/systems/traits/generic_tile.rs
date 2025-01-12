/* Imports */
use bevy::prelude::*;
use crate::{components::{planet::Planet, poi::PointOfInterestType}, systems::game::PlanetResource};

#[enum_delegate::register]
#[allow(unused_variables)]
pub trait GenericTile {
    /// Spawn logic (bevy)
    fn spawn(
        &self,
        commands: &mut ChildBuilder, // Often child of planet
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        tile_id: usize,
    ) -> Entity;

    /// What resources this tile costs
    fn cost(&self) -> Vec<(PlanetResource, usize)>;

    /// What will happen when this tile recieves
    /// energy (energy already added before this point)
    fn on_energy_recieved(&self, tile_id: usize, planet: &mut Planet) -> () {
        // Default is to do nothing
    }

    /// What POI:s this tile interacts with
    fn interacts_with(&self) -> Vec<PointOfInterestType> { Vec::new() }
}
