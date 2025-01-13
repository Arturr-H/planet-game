/* Imports */
use std::f32::consts::TAU;
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, foliage::animation::Rotate}, systems::{game::PlanetResource, traits::GenericTile}};

#[derive(Component, Clone, Debug)]
pub struct WindTurbine;
impl GenericTile for WindTurbine {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        _: &mut ResMut<Assets<TextureAtlasLayout>>,
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
                image: asset_server.load("machines/wind_turbine/stem.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            self.clone(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Visibility::Visible,
                Transform::from_translation(Vec3::new(0.0, 256.0, 0.0)),
                Rotate(2.0)
            )).with_children(|parent| {
                for i in 0..3 {
                    parent.spawn((
                        Sprite {
                            image: asset_server.load("machines/wind_turbine/fein.png"),
                            anchor: Anchor::BottomCenter,
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(0.0, 0.0, 0.1))
                            .with_scale(Vec3::splat(1.1))
                            .with_rotation(Quat::from_rotation_z(i as f32 * TAU / 3.0))
                    ));
                }
            });
        })
        .id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 2)
        ]
    }
}
