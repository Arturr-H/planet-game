/* Imports */
use std::f32::consts::PI;
use bevy::prelude::*;
use crate::{camera::OuterCamera, components::{planet::{Planet, PlayerPlanet}, poi::{PointOfInterest, PointOfInterestHighlight, PointOfInterestType}}, systems::traits::GenericTile, utils::{color::hex, logger}};
use super::{types::{battery::Battery, debug::DebugTile, drill::{Drill, DrillPlugin}, power_pole::PowerPole, solar_panel::SolarPanel, wind_turbine::WindTurbine}, Tile, TileType};

/* Constants */
const TILE_PREVIEW_ELEVATION: f32 = 10.0;

#[derive(Resource)]
pub struct TilePluginResource {
    selected: Option<(TileType, Entity)>,
    transform: Transform,
    position_index: usize,
}

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            /* Tile plugins */
            .add_plugins(DrillPlugin)

            .add_systems(Update, (Self::update, Self::update_preview))
            .insert_resource(TilePluginResource { selected: None, transform: Transform::default(), position_index: 0 });
    }
}

impl TilePlugin {
    fn update(
        mut commands: Commands,
        mut tile_plugin_resource: ResMut<TilePluginResource>,
        mut planet_q: Query<&mut Planet, With<PlayerPlanet>>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        preview_q: Query<Entity, With<TilePreview>>,
        asset_server: Res<AssetServer>,
        kb: Res<ButtonInput<KeyCode>>,
        mb: Res<ButtonInput<MouseButton>>,
    ) -> () {
        let mut planet = planet_q.single_mut();

        // Place tile
        if mb.just_pressed(MouseButton::Left) {
            if let Some((tile_type, tile_preview_entity)) = &tile_plugin_resource.selected {
                let planet_position_index = tile_plugin_resource.position_index;

                // Check if position is occupied
                let false = planet.tiles.values()
                    .any(|tile| tile.planet_position_index == planet_position_index) 
                else {
                    logger::log::red("tile_plugin", "Position is occupied");
                    return
                };

                // If we have enough resources - spend them
                if let Err(e) = planet.resources.try_spend(tile_type.cost()) {
                    logger::log::red("tile_plugin", e);
                    return
                };

                // Remove preview
                commands.entity(*tile_preview_entity).despawn_recursive();
                
                //Play sound
                let place_sound = asset_server.load("../assets/audio/place.wav");
                commands.spawn((AudioPlayer::new(place_sound), PlaybackSettings::DESPAWN));

                // Add new tile to game state
                let tile_id = planet.new_tile_id();
                planet.tiles.insert(tile_id, Tile::new(
                    tile_id,
                    planet_position_index,
                    tile_type.clone()
                ));
                commands.entity(planet.planet_entity()).with_children(|parent| {
                    tile_type.spawn(
                        parent,
                        false,
                        tile_plugin_resource.transform.with_translation(
                            tile_plugin_resource.transform.translation -
                            Planet::forward(&tile_plugin_resource.transform) * TILE_PREVIEW_ELEVATION
                        ),
                        &asset_server,
                        &mut texture_atlas_layouts,
                        tile_id
                    );
                });
                tile_plugin_resource.selected = None;
            }
        }

        let mut tile: Option<TileType> = None;
        if kb.just_pressed(KeyCode::KeyQ) { tile = Some(TileType::PowerPole(PowerPole)); }
        if kb.just_pressed(KeyCode::KeyW) { tile = Some(TileType::DebugTile(DebugTile)); }
        if kb.just_pressed(KeyCode::KeyE) { tile = Some(TileType::SolarPanel(SolarPanel)); }
        if kb.just_pressed(KeyCode::KeyR) { tile = Some(TileType::Drill(Drill)); }
        if kb.just_pressed(KeyCode::KeyT) { tile = Some(TileType::Battery(Battery)); }
        if kb.just_pressed(KeyCode::KeyY) { tile = Some(TileType::WindTurbine(WindTurbine)); }
        if kb.just_pressed(KeyCode::Escape) {
            for entity in preview_q.iter() { commands.entity(entity).despawn(); }
            tile_plugin_resource.selected = None;
        }

        if let Some(tile) = tile {
            // Remove previews
            for entity in preview_q.iter() { commands.entity(entity).despawn(); }
            let mut tile_entity = None;

            // Spawn preview 
            commands.entity(planet.planet_entity()).with_children(|parent| {
                tile_entity = Some(tile.spawn(
                    parent,
                    true,
                    Transform::default(),
                    &asset_server,
                    &mut texture_atlas_layouts,
                    usize::MAX
                ));
            });

            // Insert preview component
            if let Some(tile_entity) = tile_entity {
                commands.entity(tile_entity).insert(TilePreview);
                tile_plugin_resource.selected = Some((tile, tile_entity));
            }
        }
    }

    /// Gets the cursor position and updates the transform
    /// of the preview tile. (rotating around world matching
    /// cursor pos)
    fn update_preview(
        mut commands: Commands,
        mut query: Query<&mut Transform, With<TilePreview>>,
        mut tile_plugin_resource: ResMut<TilePluginResource>,
        planet_q: Query<(&Planet, &Transform), (With<Planet>, With<PlayerPlanet>, Without<TilePreview>)>,
        windows_q: Query<&Window>,
        camera_q: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    ) -> () {
        let window = windows_q.single();
        let (camera, camera_transform) = camera_q.single();
        let (planet, planet_transform) = planet_q.single();
        let planet_rotation_z = planet_transform.rotation.to_euler(EulerRot::XYZ).2 - PI / 2.0;
        let planet_pos = planet_transform.translation.truncate();

        let Some((tile_type, _)) = &tile_plugin_resource.selected else { return };
        let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
            else { return };
        let Ok(mut transform) = query.get_single_mut() else { return };

        let cursor_angle = (cursor_pos - planet_pos).angle_to(Vec2::Y);
        let index = planet.radians_to_index(- planet_rotation_z - cursor_angle);
        let p = planet.index_to_transform(index, TILE_PREVIEW_ELEVATION, 2.0);

        if !tile_type.interacts_with().is_empty() {
            for pos_index in planet.numbers_in_radius(index, 2) {
                let Some(pois) = planet.points_of_interest.get(&pos_index) else { continue };
                'inner: for poi in pois {
                    if !tile_type.interacts_with().contains(&poi.poi_type) { continue };
                    let Some(mut entity) = commands.get_entity(poi.entity) else { continue 'inner };
                    entity.insert(PointOfInterestHighlight::new());
                }
            };
        }

        tile_plugin_resource.transform = *transform;
        tile_plugin_resource.position_index = index;

        //Offset the placement with 1 unit to make sure the object is wedged into the ground
        transform.translation = p.translation.with_z(-0.4) - Planet::forward(&transform) * 1.; 
        transform.rotation = p.rotation;
    }

    fn snap(angle: f32, snap_to: f32) -> f32 {
        let two_pi = std::f32::consts::TAU;
        let normalized = angle % two_pi;
        let wrapped = if normalized < 0.0 { normalized + two_pi } else { normalized };
    
        // Calculate the closest snap point
        let snap_count = (two_pi / snap_to).round();
        let step = two_pi / snap_count;
    
        (wrapped / step).round() * step % two_pi
    }
    fn snap_index(angle: f32, snap_to: f32) -> usize {
        let two_pi = std::f32::consts::PI * 2.0;
        let normalized = angle.rem_euclid(two_pi); // Normalize to [0, 2π)
        let snap_count = (two_pi / snap_to).round() as usize; // Number of snap points
        let step = two_pi / snap_count as f32; // Angle per snap point
    
        (normalized / step).round() as usize % snap_count
    }
}

#[derive(Component)]
pub struct TilePreview;
