/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, planet::Planet, poi::{stone::Stone, PointOfInterestType}, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}, utils::{audio::play_audio, logger}};

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
        spawn_params: &mut TileSpawnEventParams,
        spawn_data: &TileSpawnEvent,
    ) -> Entity {
        let transform = spawn_params.planet.index_to_transform(
            spawn_data.tile.tile_id, 0.0, 1.0, spawn_data.tile.tile_type.width()
        );

        if !spawn_data.is_preview {
            CableSlot::spawn(
                commands, &spawn_params.asset_server, spawn_data.tile.tile_id, transform
                    .with_translation(transform.translation.with_z(2.0)
                        + Planet::forward(&transform) * 20.0)
            );
        }

        let texture = spawn_params.asset_server.load("machines/drill.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
        let texture_atlas_layout = spawn_params.texture_atlas_layouts.add(layout);
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
        )).id()
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 4)
        ]
    }

    fn display_name(&self) -> String { "Drill".to_string() }
    fn can_recieve_energy(&self) -> bool { true }

    fn on_tick(&self, tile_id: usize, planet: &mut Planet) -> () {
        // The position index of the drill
        let position_index = planet.tiles[&tile_id].tile_id.clone();

        // Must be at least DRILL_RANGE tile indexes
        // away from the POI to drill it.
        for poi_pos_index in planet.numbers_in_radius(position_index, DRILL_RANGE) {
            let Some(local_pois) = planet.points_of_interest.get(&poi_pos_index) else { continue; };

            for poi in local_pois {
                match poi.poi_type {
                    PointOfInterestType::Stone(_) => {
                        planet.tiles.get_mut(&tile_id).map(|e| {
                            // If we have enough energy, mine one stone.
                            if e.powergrid_status.energy_stored >= 5.0 {
                                e.powergrid_status.energy_stored -= 5.0;
                                planet.resources.add(PlanetResource::Stone, 1);
                            }
                        });
                    },
                    _ => {}
                };
            }
        }
    }

    fn interacts_with(&self) -> Vec<PointOfInterestType> {
        vec![PointOfInterestType::Stone(Stone)]
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
