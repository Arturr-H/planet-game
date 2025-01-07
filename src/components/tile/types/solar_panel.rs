/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::cable::slot::CableSlot, systems::{game::PlanetResource, traits::GenericTile}};

/// A solar panel is a tile that generates energy
/// if sun is shining on it.
#[derive(Component, Clone, Debug)]
pub struct SolarPanel;
impl GenericTile for SolarPanel {
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
                image: asset_server.load("machines/solar-panel.png"),
                anchor: Anchor::BottomCenter,
                // custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            SolarPanel,
            PIXEL_PERFECT_LAYERS,
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 4)
        ]
    }
}
