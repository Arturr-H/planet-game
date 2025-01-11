/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, planet::Planet}, systems::{game::PlanetResource, traits::GenericTile}};

/* Constants */
const POWER_SLOT_OFFSET: f32 = 50.0;
// const POLE_GROUND_INSERTION: f32 = -15.0; // How much the pole is inserted into the ground

/// Has a cable slot for keeping cables connected (and above ground)
#[derive(Component, Clone, Debug)]
pub struct PowerPole;
impl GenericTile for PowerPole {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        _: &mut ResMut<Assets<TextureAtlasLayout>>,
        tile_id: usize,
    ) -> Entity {

        /* Power pole sprite */
        let id = commands.spawn((
            Sprite {
                image: asset_server.load("machines/power-pole.png"),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform,
            PowerPole,
        )).id();

        if !preview {
            CableSlot::spawn(
                commands, asset_server, tile_id,
                transform.with_translation(transform.translation
                    + Planet::forward(&transform) * POWER_SLOT_OFFSET
                )
            );
        }

        id
    }

    fn cost(&self) -> Vec<(PlanetResource,usize)> {
        vec![
            (PlanetResource::Wood, 6)
        ]
    }
}
