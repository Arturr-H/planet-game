/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::Planet}, systems::{game::{GameState, PlanetResource}, traits::{GenericTile, PowergridStatus}}, utils::color::hex};

/// Drills rocks sometimes I think...
#[derive(Component, Clone, Debug)]
pub struct Drill;
impl GenericTile for Drill {
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
                    .with_translation(transform.translation.with_z(2.0)
                        + Planet::forward(&transform) * 20.0)
            );
        }

        commands.spawn((
            transform,
            Sprite {
                image: asset_server.load("machines/drill.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Drill,
            PIXEL_PERFECT_LAYERS,
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 4)
        ]
    }
}
