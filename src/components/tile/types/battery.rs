/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::cable::slot::CableSlot, systems::{game::PlanetResource, traits::GenericTile}};

#[derive(Component, Clone, Debug)]
pub struct Battery;
impl GenericTile for Battery {
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
                image: asset_server.load("machines/battery.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            self.clone(),
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 2)
        ]
    }
}
