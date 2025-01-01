/* Imports */
use bevy::prelude::*;

/// The status of the connection to the power grid
#[derive(Component, Clone)]
pub struct PowergridStatus {
    pub connected_tiles: Vec<usize>,
}

impl Default for PowergridStatus {
    fn default() -> Self {
        Self {
            connected_tiles: Vec::new(),
        }
    }
}
