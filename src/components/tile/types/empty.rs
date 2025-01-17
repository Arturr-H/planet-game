/* Imports */
use bevy::prelude::*;
use crate::{components::{cable::slot::CableSlot, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}, utils::color::hex};

#[derive(Component, Clone, Debug)]
pub struct EmptyTile;

impl GenericTile for EmptyTile {
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
                color: hex!("#ffffff00"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            self.clone(),
        )).id()
    }

    fn display_name(&self) -> String {
        "Empty".to_string()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        Vec::new()
    }
}
