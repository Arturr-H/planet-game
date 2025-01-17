/* Imports */
use std::f32::consts::TAU;
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, foliage::animation::Rotate, planet::Planet, tile::{spawn::{TileSpawnEvent, TileSpawnEventParams}, TileType}}, systems::{game::PlanetResource, traits::GenericTile}};

/* Constants */
const CABLE_SLOT_OFFSET: f32 = 28.0;

#[derive(Component, Clone, Debug)]
pub struct WindTurbine;
impl GenericTile for WindTurbine {
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
                    .with_translation(transform.translation
                        .with_z(transform.translation.z + 0.1)
                        + Planet::forward(&transform) * CABLE_SLOT_OFFSET)
            );
        }

        commands.spawn((
            transform,
            Visibility::Visible,
            self.clone(),
        ))
        .with_children(|parent| {
            /* Wind turbine "stem" */
            parent.spawn((
                Sprite {
                    image: spawn_params.asset_server.load("machines/wind_turbine/stem.png"),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
            ));

            /* Rotorblades or whatever they're called */
            parent.spawn((
                Sprite {
                    image: spawn_params.asset_server.load("machines/wind_turbine/rotors.png"),
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, 256.0, 0.05)),
                Rotate(2.0),
            ));

            /* The circle in the middle */
            parent.spawn((
                Sprite {
                    image: spawn_params.asset_server.load("machines/wind_turbine/knob.png"),
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, 256.0, 0.10)),
            ));
        })
        .id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 2)
        ]
    }

    fn display_name(&self) -> String {
        "Wind turbine".to_string()
    }

    // So wind turbine rotors don't overlap
    fn keep_distance_from(&self) -> Vec<(usize,crate::components::tile::TileType)> {
        vec![
            (8, TileType::WindTurbine(WindTurbine))
        ]
    }
}
