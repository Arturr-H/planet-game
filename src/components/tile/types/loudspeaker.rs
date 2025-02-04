/* Imports */
use bevy::{audio::Volume, prelude::*, sprite::Anchor};
use rand::Rng;
use crate::{components::{cable::slot::CableSlot, planet::Planet, poi::{copper::Copper, stone::Stone, PointOfInterestType}, tile::spawn::{TileSpawnEvent, TileSpawnEventParams}}, systems::{game::PlanetResource, traits::GenericTile}, utils::{audio::{game_sounds, play_audio, PlayAudioEvent}, logger}};

/* Constants */

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Clone, Debug)]
pub struct Loudspeaker;
impl GenericTile for Loudspeaker {
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
            play_audio(
                game_sounds::DRILL,
                PlaybackSettings {mode: bevy::audio::PlaybackMode::Loop, volume: Volume::new(0.1),spatial: true, ..Default::default() },
                Some(transform.translation),
                &mut spawn_params.audio_events
            );
        }

        let texture = spawn_params.asset_server.load("machines/loudspeaker.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 48), 15, 1, None, None);
        let texture_atlas_layout = spawn_params.texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 14 };

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
            Loudspeaker,
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
    fn display_name(&self) -> String { "Loudspeaker".to_string() }
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

pub struct LoudspeakerPlugin;
impl Plugin for LoudspeakerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, animate_sprite);
    }
}
