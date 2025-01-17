/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}};

#[derive(Component, Clone, Debug)]
pub struct LandedRocket;

impl GenericTile for LandedRocket {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        spawn_params: &mut TileSpawnEventParams,
        spawn_data: &TileSpawnEvent,
    ) -> Entity {
        let transform = spawn_params.planet.index_to_transform(
            spawn_data.tile_id, 0.0, 1.0);
        
        if !spawn_data.is_preview {
            CableSlot::spawn(
                commands, &spawn_params.asset_server, spawn_data.tile_id, transform
            );
        }

        commands.spawn((
            transform,
            Sprite {
                image: spawn_params.asset_server.load("machines/rocketship.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            self.clone(),
        )).id()
    }

    fn display_name(&self) -> String {
        "Landed rocket".to_string()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        Vec::new()
    }
}
