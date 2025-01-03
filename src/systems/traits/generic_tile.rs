/* Imports */
use bevy::prelude::*;
use crate::systems::{game::GameState, traits::PowergridStatus};

#[enum_delegate::register]
pub trait GenericTile {
    /// Spawn logic (bevy)
    fn spawn(
        &self,
        commands: &mut ChildBuilder, // Often child of planet
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        tile_id: usize,
    ) -> Entity;
}
