/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, planet::Planet, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}};

#[derive(Component, Clone, Debug)]
pub struct Battery;
impl GenericTile for Battery {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        spawn_params: &mut TileSpawnEventParams,
        spawn_data: &TileSpawnEvent,
    ) -> Entity {
        let transform = spawn_params.planet.index_to_transform(
            spawn_data.tile_id, 0.0, 1.0, spawn_data.tile_type.width());
        if !spawn_data.is_preview {
            CableSlot::spawn(
                commands, &spawn_params.asset_server, spawn_data.tile_id, transform
                    .with_translation(transform.translation
                        + Planet::forward(&transform) * 38.0)
            );
        }

        commands.spawn((
            transform,
            Sprite {
                image: spawn_params.asset_server.load("machines/battery.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            self.clone(),
        )).id()
    }

    fn display_name(&self) -> String {
        "Battery".to_string()
    }


    fn width(&self) -> usize {
        2
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 2)
        ]
    }
}
