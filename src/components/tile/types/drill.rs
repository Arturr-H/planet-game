/* Imports */
use bevy::{audio::Volume, prelude::*, sprite::Anchor};
use rand::Rng;
use crate::{components::{cable::slot::CableSlot, planet::Planet, poi::{copper::Copper, stone::Stone, PointOfInterestType}, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}, utils::{audio::{game_sounds, play_audio, PlayAudioEvent}, logger}};

/* Constants */
/// How many tiles to the left and the right
/// this drill will find e.g rocks to drill.
pub const DRILL_RANGE: usize = 0;

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

            play_audio(
                game_sounds::DRILL,
                PlaybackSettings {mode: bevy::audio::PlaybackMode::Loop, volume: Volume::new(0.1),spatial: true, ..Default::default() },
                Some(transform.translation),
                &mut spawn_params.audio_events
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
    fn width(&self) -> usize {
        2
    }
    fn display_name(&self) -> String { "Drill".to_string() }
    fn can_recieve_energy(&self) -> bool { true }

    fn on_tick(&self, tile_id: usize, planet: &mut Planet, audio_events: &mut EventWriter<PlayAudioEvent>) {
        let position_index = planet.tiles[&tile_id].tile_id;
        let mut pois_in_range = Vec::new();
    
        for poi_pos_index in planet.numbers_in_radius(position_index, DRILL_RANGE) {
            if let Some(local_pois) = planet.points_of_interest.get(&poi_pos_index) {
                pois_in_range.extend(local_pois.iter().cloned());
            }
        }
    
        if !pois_in_range.is_empty() {
            let mut rng = rand::thread_rng();
            let selected_poi = &pois_in_range[rng.gen_range(0..pois_in_range.len())];
            
            let (has_energy, width) = match planet.tiles.get_mut(&tile_id) {
                Some(tile) if tile.powergrid_status.energy_stored >= 5.0 => {
                    tile.powergrid_status.energy_stored -= 5.0;
                    (true, tile.tile_type.width())
                },
                _ => (false, 0),
            };
    
            if !has_energy { return; } // break if no energy
    
            match selected_poi.poi_type {
                PointOfInterestType::Stone(_) => {
                    planet.resources.add(PlanetResource::Stone, 1);
                    let transform = planet.index_to_transform(position_index, 0.0, 1.0, width);
                    play_audio(
                        game_sounds::stone::DAMAGE,
                        PlaybackSettings {
                            volume: Volume::new(0.4),
                            spatial: true,
                            ..Default::default()
                        },
                        Some(transform.translation),
                        audio_events
                    );
                },
                PointOfInterestType::Copper(_) => {
                    planet.resources.add(PlanetResource::Copper, 1);
                    let transform = planet.index_to_transform(position_index, 0.0, 1.0, width);
                    play_audio(
                        game_sounds::stone::DAMAGE,
                        PlaybackSettings {
                            volume: Volume::new(0.4),
                            spatial: true,
                            ..Default::default()
                        },
                        Some(transform.translation),
                        audio_events
                    );
                },
                _ => {}
            }
        }
    }

    fn interacts_with(&self) -> Vec<PointOfInterestType> {
        vec![PointOfInterestType::Stone(Stone), PointOfInterestType::Copper(Copper)]
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
