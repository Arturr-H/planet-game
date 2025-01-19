/* Imports */
use bevy::{prelude::*, state::state::setup_state_transitions_in_world};
use crate::{components::{cable::slot::{CableSlot, RemoveAllCableSlotHighlightsCommand, RemoveCableSlotCommand}, planet::{Planet, PlayerPlanet}}, systems::{game::GameState, traits::GenericTile}, utils::logger};
use super::spawn::TileSpawnEvent;

/* Constants */

#[derive(Clone)]
pub struct UpgradeTileCommand {
    pub tile_id: usize,
}

impl Command for UpgradeTileCommand {
    fn apply(self, world: &mut World) {
        let Self { tile_id } = self.clone();
        
        let mut query_state = world.query_filtered::<&mut Planet, With<PlayerPlanet>>();
        if let Ok(planet) = query_state.get_single_mut(world) {
            /* Remove cables */
            let Some(tile) = planet.tiles.get(&tile_id).cloned() else { return };

            /* Remove cable previews and other highlights */
            RemoveAllCableSlotHighlightsCommand.apply(world);

            /* Despawn tile & its children */
            DespawnRecursive { entity: tile.entity, warn: true }.apply(world);

            /* Despawn slots related to the tile if they exist */
            RemoveCableSlotCommand { tile_id }.apply(world);

            /* Spawn new tile */
            logger::log::bright_green("tile_upgrade",
                &format!("Upgraded tile {:?} at index {}", tile.tile_type.display_name(), tile.tile_id));
            world.resource_mut::<Events<TileSpawnEvent>>()
                .send(TileSpawnEvent { tile: tile.clone(), is_preview: false, upgrade: true });
        }
    }
}
