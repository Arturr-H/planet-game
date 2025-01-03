use std::f32::consts::PI;

/* Imports */
use bevy::prelude::*;
use crate::{camera::OuterCamera, components::planet::planet::Planet, systems::{game::GameState, traits::GenericTile}};
use super::{debug::DebugTile, power_pole::PowerPole, solar_panel::SolarPanel, Tile, TileType};

#[derive(Resource)]
pub struct TilePluginResource {
    selected: Option<(TileType, Entity)>,
    transform: Transform
}

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (Self::update, Self::update_preview))
            .insert_resource(TilePluginResource { selected: None, transform: Transform::default() });
    }
}

impl TilePlugin {
    fn update(
        mut commands: Commands,
        mut tile_plugin_resource: ResMut<TilePluginResource>,
        mut game_state: ResMut<GameState>,
        preview_q: Query<Entity, With<TilePreview>>,
        asset_server: Res<AssetServer>,
        kb: Res<ButtonInput<KeyCode>>,
        mb: Res<ButtonInput<MouseButton>>,
    ) -> () {
        /* Place tile */
        if mb.just_pressed(MouseButton::Left) {
            if let Some((tile_type, tile_entity)) = &tile_plugin_resource.selected {
                let tile_id = game_state.new_cable_slot_id();
                commands.entity(*tile_entity).despawn();

                // Add new tile to game state
                game_state.tiles.insert(tile_id, Tile::new(tile_id, tile_type.clone()));

                commands.entity(game_state.planet_entity()).with_children(|parent| {
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
            commands.entity(game_state.planet_entity()).with_children(|parent| {
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
        planet_q: Query<&Transform, (With<Planet>, Without<TilePreview>)>,
        windows_q: Query<&Window>,
        camera_q: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    ) -> () {
        let window = windows_q.single();
        let (camera, camera_transform) = camera_q.single();
        let planet = planet_q.single();
        let planet_rotation_z = planet.rotation.to_euler(EulerRot::XYZ).2 - PI / 2.0;
        let planet_pos = planet.translation.truncate();

        if let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            let angle = (cursor_pos - planet_pos).angle_to(Vec2::Y);

            if let Ok(mut transform) = query.get_single_mut() {
                tile_plugin_resource.transform = *transform;
                let degree = - planet_rotation_z - angle;
                let p = Planet::degree_to_transform(degree, -8.0, 2.0);
                transform.translation = p.translation.with_z(-0.4);
                transform.rotation = p.rotation;
            }
        }
    }
}

#[derive(Component)]
pub struct TilePreview;
