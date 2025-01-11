/* Imports */
use bevy::prelude::*;
use crate::{components::cable::slot::CableSlot, systems::{game::PlanetResource, traits::GenericTile}, utils::color::hex};

#[derive(Component, Clone, Debug)]
pub struct EmptyTile;

impl GenericTile for EmptyTile {
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
                color: hex!("#ffffff00"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            self.clone(),
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        Vec::new()
    }
}
