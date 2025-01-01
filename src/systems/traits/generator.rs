/* Imports */
use bevy::{prelude::*, utils::HashMap};
use crate::systems::game::GameState;
use super::GenericTile; 

/// The generator trait is used by
/// all energy generators.
pub trait Generator: GenericTile {
    /// The amount of energy generated per tick.
    fn output(&self) -> f32;

    /// Distribute energy across cables from this tile.
    /// Only runs from generators.
    fn distribute_energy(&self, game_state: &mut GameState) -> () {
        // HashMap<slot_id, will_recieve_energy>
        let mut visited: HashMap<usize, bool> = HashMap::new();
        let mut recievers = 0;

        search(
            &mut recievers,
            game_state,
            self.slot_id(),
            &mut visited
        );

        let energy_per_reciever = self.output() / recievers.max(1) as f32;
        for (slot_id, will_recieve_energy) in visited {
            if will_recieve_energy {
                match game_state.slots.get_mut(&slot_id) {
                    Some(e) => e.add_energy(energy_per_reciever),
                    None => ()
                };
            }
        }
    }
}

fn search(
    recievers: &mut usize,
    game_state: &GameState,
    tile_id: usize,
    visited: &mut HashMap<usize, bool>,
) -> () {
    let Some(tile) = game_state.slots.get(&tile_id) else { return };
    visited.insert(tile_id, tile.can_store_energy());
    if tile.can_store_energy() {
        *recievers += 1;
    }

    for tile_id in &tile.powergrid_status().connected_tiles {
        if !visited.contains_key(tile_id) {
            search(recievers, game_state, *tile_id, visited);
        }
    }
}