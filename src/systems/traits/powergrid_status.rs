/* Imports */
use bevy::prelude::*;

/// The status of the connection to the power grid
#[derive(Component, Clone, Debug)]
pub struct PowergridStatus {
    pub connected_tiles: Vec<usize>,

    // Energy that is stored in this tile (won't move)
    pub energy_stored: f32,
}

impl Default for PowergridStatus {
    fn default() -> Self {
        Self {
            connected_tiles: Vec::new(),
            energy_stored: 0.0,
        }
    }
}
