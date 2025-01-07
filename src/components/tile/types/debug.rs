/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::Planet}, systems::{game::{GameState, PlanetResource}, traits::{EnergyStorage, GenericTile, PowergridStatus}}, utils::color::hex};

#[derive(Component, Clone, Debug)]
pub struct DebugTile;
impl GenericTile for DebugTile {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        _: &mut ResMut<Assets<TextureAtlasLayout>>,
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
                image: asset_server.load("machines/debug.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            self.clone(),
            PIXEL_PERFECT_LAYERS,
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 2)
        ]
    }
}
