/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::cable::slot::CableSlot, systems::{game::PlanetResource, traits::GenericTile}};

#[derive(Component, Clone, Debug)]
pub struct LandedRocket;

impl GenericTile for LandedRocket {
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
                image: asset_server.load("machines/rocketship.png"),
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
