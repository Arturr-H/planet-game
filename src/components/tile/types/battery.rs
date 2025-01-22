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

        let mesh_size = Vec2::new(32.0, 48.0);
        let outline_extend = 2.5;
        
        commands.spawn((
            transform,
            self.clone(),
        ))
        .with_children(|parent| {
            // let outline_offset = Planet::forward(&transform) * (mesh_size.y / 2.0);
            parent.spawn((
                Sprite {
                    image: spawn_params.asset_server.load("machines/battery.png"),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                self.clone(),
            ));            

            parent.spawn((
                Mesh2d(spawn_params.meshes.add(Rectangle::new(mesh_size.x + outline_extend, mesh_size.y + outline_extend))),
                MeshMaterial2d (spawn_params.outline_material.add(TileMaterialOutline{
                    color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
                    // thickness: 0.1,
                    texture: spawn_params.asset_server.load("machines/battery.png")
                })),    
                Transform::from_translation(Vec3::new(
                    0.0,
                    mesh_size.y / 2.0,
                    -0.1
                )),
                Visibility::Hidden,
            ));
        }).id()
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