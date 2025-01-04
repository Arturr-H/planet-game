use std::f32::consts::{PI, TAU};

/* Imports */
use bevy::prelude::*;
use crate::{camera::OuterCamera, components::planet::planet::{Planet, PlayerPlanet}, systems::{game::GameState, traits::GenericTile}, utils::logger};
use super::{debug::DebugTile, power_pole::PowerPole, solar_panel::SolarPanel, Tile, TileType, TILE_SIZE};

#[derive(Resource)]
pub struct TilePluginResource {
    selected: Option<(TileType, Entity)>,
    transform: Transform,
    degree: f32,
}

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (Self::update, Self::update_preview))
            .insert_resource(TilePluginResource { selected: None, transform: Transform::default(), degree: 0.0 });
    }
}

impl TilePlugin {
    fn update(
        mut commands: Commands,
        mut tile_plugin_resource: ResMut<TilePluginResource>,
        mut planet_q: Query<&mut Planet, With<PlayerPlanet>>,
        preview_q: Query<Entity, With<TilePreview>>,
        asset_server: Res<AssetServer>,
        kb: Res<ButtonInput<KeyCode>>,
        mb: Res<ButtonInput<MouseButton>>,
    ) -> () {
        let mut planet = planet_q.single_mut();

        // Place tile
        if mb.just_pressed(MouseButton::Left) {
            if let Some((tile_type, tile_preview_entity)) = &tile_plugin_resource.selected {
                let planet_position_index = Self::snap_index(tile_plugin_resource.degree, planet.angular_step());

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
                commands.entity(*tile_preview_entity).despawn();
                
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
                        tile_plugin_resource.transform,
                        &asset_server,
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

        if let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            let angle = (cursor_pos - planet_pos).angle_to(Vec2::Y);

            if let Ok(mut transform) = query.get_single_mut() {
                let degree = Self::snap(- planet_rotation_z - angle, planet.angular_step());
                let p = planet.degree_to_transform(degree, 0.0, 2.0);

                tile_plugin_resource.transform = *transform;
                tile_plugin_resource.degree = degree;

                transform.translation = p.translation.with_z(-0.4);
                transform.rotation = p.rotation;
            }
        }
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
