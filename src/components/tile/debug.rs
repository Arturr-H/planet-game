/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::planet::Planet}, systems::{game::GameState, traits::{EnergyStorage, GenericTile, PowergridStatus}}, utils::color::hex};
use super::{empty::EmptyTile, Tile, TileType};

#[derive(Component, Clone, Debug)]
pub struct DebugTile;
impl GenericTile for DebugTile {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        tile_id: usize,
    ) -> Entity {
        if !preview {
            CableSlot::spawn(
                commands, asset_server, tile_id, transform
            );
        }

        commands.spawn((
            transform,
            Sprite {
                color: hex!("#ff0000"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            self.clone(),
            PIXEL_PERFECT_LAYERS,
        )).id()
    }
}
