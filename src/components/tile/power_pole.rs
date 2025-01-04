/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::planet::Planet}, systems::{game::{GameState, PlanetResource}, traits::{GenericTile, PowergridStatus}}, utils::color::hex};
use super::{empty::EmptyTile, solar_panel::SolarPanel, Tile, TileType};

/* Constants */
const POWER_SLOT_OFFSET: f32 = 50.0;
const POLE_GROUND_INSERTION: f32 = -15.0; // How much the pole is inserted into the ground

/// Has a cable slot for keeping cables connected (and above ground)
#[derive(Component, Clone, Debug)]
pub struct PowerPole;
impl GenericTile for PowerPole {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        tile_id: usize,
    ) -> Entity {
        let translation = transform.translation.with_z(-0.4)
            + Planet::forward(&transform) * POLE_GROUND_INSERTION;

        /* Power pole sprite */
        let id = commands.spawn((
            Sprite {
                image: asset_server.load("machines/power-pole.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform.with_translation(translation),
            PowerPole,
            PIXEL_PERFECT_LAYERS,
        )).id();

        if !preview {
            CableSlot::spawn(
                commands, asset_server, tile_id,
                transform.with_translation(translation
                    + Planet::forward(&transform) * POWER_SLOT_OFFSET
                )
            );
        }

        id
    }

    fn cost(&self) -> Vec<(PlanetResource,usize)> {
        vec![
            (PlanetResource::Wood, 6)
        ]
    }
}
