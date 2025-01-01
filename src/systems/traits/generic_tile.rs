/* Imports */
use bevy::prelude::*;
use crate::systems::{game::GameState, traits::PowergridStatus};

#[enum_delegate::register]
pub trait GenericTile {
    /// What slot this tile is placed in
    fn slot_id(&self) -> usize;

    /// Spawn logic (bevy)
    fn spawn(&self, commands: &mut ChildBuilder, transform: Transform, asset_server: &Res<AssetServer>, game_state: &mut ResMut<GameState>) -> ();

    /// Get the power grid status of this tile
    fn powergrid_status(&self) -> &PowergridStatus;
    fn powergrid_status_mut(&mut self) -> &mut PowergridStatus;
}
