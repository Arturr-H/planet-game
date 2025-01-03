/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::planet::Planet}, systems::{game::GameState, traits::{GenericTile, PowergridStatus}}, utils::color::hex};
use super::{empty::EmptyTile, Tile, TileType};

/// A solar panel is a tile that generates energy
/// if sun is shining on it.
#[derive(Component, Clone, Debug)]
pub struct SolarPanel;
impl GenericTile for SolarPanel {
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
                color: hex!("#ffff00"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            SolarPanel,
            PIXEL_PERFECT_LAYERS,
        )).id()
    }
}
