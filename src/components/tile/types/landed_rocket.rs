/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::planet::Planet}, systems::{game::{GameState, PlanetResource}, traits::{EnergyStorage, GenericTile, PowergridStatus}}, utils::color::hex};

#[derive(Component, Clone, Debug)]
pub struct LandedRocket;

impl GenericTile for LandedRocket {
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
                image: asset_server.load("machines/rocketship.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            self.clone(),
            PIXEL_PERFECT_LAYERS,
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        Vec::new()
    }
}
