/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::slot::CableSlot, planet::{Planet, PlanetPointOfInterest}}, systems::{game::{GameState, PlanetResource}, traits::{GenericTile, PowergridStatus}}, utils::{color::hex, logger}};

/* Constants */
/// How many tiles to the left and the right
/// this drill will find e.g rocks to drill.
pub const DRILL_RANGE: usize = 2;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

/// Drills rocks sometimes I think...
#[derive(Component, Clone, Debug)]
pub struct Drill;
impl GenericTile for Drill {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        preview: bool,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        tile_id: usize,
    ) -> Entity {
        if !preview {
            CableSlot::spawn(
                commands, asset_server, tile_id, transform
                    .with_translation(transform.translation.with_z(2.0)
                        + Planet::forward(&transform) * 20.0)
            );
        }

        let texture = asset_server.load("machines/drill.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

        commands.spawn((
            transform,
            Sprite {
                anchor: Anchor::BottomCenter,
                image: texture,
                texture_atlas: Some(
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first
                    }
                ),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animation_indices,
            Drill,
            PIXEL_PERFECT_LAYERS,
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 4)
        ]
    }

    fn on_energy_recieved(&self, tile_id: usize, planet: &mut Planet) -> () {
        let position_index = planet.tiles[&tile_id].planet_position_index.clone();

        for (poi_pos_index, poi_type) in planet.points_of_interest.iter() {
            // Must be at least DRILL_RANGE tile indexes
            // away from the POI to drill it.
            if !planet.in_range(position_index, *poi_pos_index, DRILL_RANGE) { break; }
            match poi_type {
                &PlanetPointOfInterest::Stone => planet.tiles.get_mut(&tile_id).map(|e| {
                    // If we have enough energy, mine one stone.
                    logger::log::bright_red("drill", format!("Stored: {}", e.powergrid_status.energy_stored));
                    if e.powergrid_status.energy_stored >= 5.0 {
                        e.powergrid_status.energy_stored = 0.0;
                        logger::log::bright_red("drill", "Mined one stone");
                        planet.resources.add(PlanetResource::Stone, 1);
                    }
                })
            };
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub struct DrillPlugin;
impl Plugin for DrillPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, animate_sprite);
    }
}
