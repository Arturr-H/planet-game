/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, planet::Planet, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}};

/* Constants */
const POWER_SLOT_OFFSET: f32 = 50.0;
// const POLE_GROUND_INSERTION: f32 = -15.0; // How much the pole is inserted into the ground

/// Has a cable slot for keeping cables connected (and above ground)
#[derive(Component, Clone, Debug)]
pub struct PowerPole;
impl GenericTile for PowerPole {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        spawn_params: &mut TileSpawnEventParams,
        spawn_data: &TileSpawnEvent,
    ) -> Entity {
        let transform = spawn_params.planet.index_to_transform(
            spawn_data.tile.tile_id, 0.0, 1.0, spawn_data.tile.tile_type.width());

        /* Power pole sprite */
        let id = commands.spawn((
            Sprite {
                image: spawn_params.asset_server.load("machines/power-pole.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform,
            PowerPole,
        )).id();

        if !spawn_data.is_preview {
            CableSlot::spawn(
                commands, &spawn_params.asset_server, spawn_data.tile.tile_id,
                transform.with_translation(transform.translation
                    + Planet::forward(&transform) * POWER_SLOT_OFFSET
                )
            );
        }

        id
    }

    fn display_name(&self) -> String {
        "Power pole".to_string()
    }

    fn cost(&self) -> Vec<(PlanetResource,usize)> {
        vec![
            (PlanetResource::Wood, 6)
        ]
    }
}
