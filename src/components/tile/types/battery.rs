/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, planet::Planet, tile::{material::TileMaterialOutline, spawn::{TileSpawnEvent, TileSpawnEventParams}}}, systems::{game::PlanetResource, traits::GenericTile}};

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
            spawn_data.tile.tile_id, 0.0, 1.0, spawn_data.tile.tile_type.width());
        if !spawn_data.is_preview {
            CableSlot::spawn(
                commands, &spawn_params.asset_server, spawn_data.tile.tile_id, transform
                    .with_translation(transform.translation
                        + Planet::forward(&transform) * 38.0)
            );
        }

        

        commands.spawn((
            Mesh2d(spawn_params.meshes.add(Rectangle::new(32.0, 48.0))), //32.0, 48.0
            transform,
            MeshMaterial2d (spawn_params.outline_material.add(TileMaterialOutline{
                color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
                thickness: 0.01,
                texture: spawn_params.asset_server.load("machines/battery.png"
            )})),
            self.clone(),
        )).id()
    }

    fn display_name(&self) -> String { "Battery".to_string() }
    fn width(&self) -> usize { 2 }
    fn can_recieve_energy(&self) -> bool { true }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 2)
        ]
    }
}
