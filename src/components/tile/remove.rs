/* Imports */
use bevy::{prelude::*, state::state::setup_state_transitions_in_world};
use crate::{components::{cable::slot::{CableSlot, RemoveAllCableSlotHighlightsCommand, RemoveCableSlotCommand}, planet::{Planet, PlayerPlanet}}, systems::game::GameState, utils::logger};

/* Constants */

#[derive(Clone)]
pub struct RemoveTileCommand {
    pub tile_id: usize,
}

impl Command for RemoveTileCommand {
    fn apply(self, world: &mut World) {
        let Self { tile_id } = self.clone();

        let mut query_state = world.query_filtered::<&mut Planet, With<PlayerPlanet>>();
        let mut tile_entity = None;
        if let Ok(mut planet) = query_state.get_single_mut(world) {
            
            /* Remove cables */
            let Some(tile) = planet.tiles.get(&tile_id) else { return };
            tile_entity = Some(tile.entity);

            for cable_id in tile.powergrid_status.connected_tiles.clone().iter() {
                // `tile_with_cables` is a tile that has
                // cables connected to the current tile
                if let Some(tile_with_cables) = planet.tiles.get_mut(cable_id) {
                    tile_with_cables.powergrid_status.connected_tiles.retain(|&id| id != tile_id);
                }
            }

            /* Remove tile */
            planet.tiles.remove(&tile_id);
        }

        /* Remove cable previews and other highlights */
        println!("!hAHA");
        RemoveAllCableSlotHighlightsCommand.apply(world);

        /* Despawn tile & its children */
        if let Some(entity) = tile_entity {
            DespawnRecursive { entity, warn: true }.apply(world);
        }

        /* Despawn slots related to the tile if they exist */
        RemoveCableSlotCommand { tile_id }.apply(world);
    }
}
