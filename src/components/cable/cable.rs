/* Imports */
use bevy::prelude::*;
use crate::{camera::OuterCamera, utils::color::hex};
use super::slot::CableSlot;

/* Constants */
const CABLE_Z_INDEX: f32 = 3.0;
const CABLE_THICKNESS: f32 = 1.25;
const CABLE_COLOR: &str = "#020410";
pub const MAX_CABLE_LENGTH: f32 = 200.0;

/// Plugin to add cable rendering functionality
pub struct CablePlugin;
impl Plugin for CablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (Cable::update_cables, Cable::update_previews));
    }
}

/// Component to represent a cable between two entities
#[derive(Component)]
pub struct Cable {
    start_entity: Entity,
    end_entity: Entity,
}

/// Used to highlight the cable between two entities
/// before the cable is actually spawned
#[derive(Component)]
pub struct CablePreview {
    start_entity: Entity
}

impl Cable {
    /// Spawn a cable between two entities
    pub fn spawn_between_slots(
        commands: &mut ChildBuilder,
        start_entity: Entity,
        end_entity: Entity,
    ) {
        commands.spawn((
            PickingBehavior::IGNORE,
            Sprite {
                color: hex!(CABLE_COLOR),
                custom_size: Some(Vec2::new(1.0, CABLE_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, CABLE_Z_INDEX),
            Cable {
                start_entity,
                end_entity,
            },
        ));
    }

    /// Update system
    fn update_cables(
        mut query: Query<(&mut Transform, &Cable)>,
        slots_q: Query<&Transform, (With<CableSlot>, Without<Cable>)>,
    ) {
        for (mut transform, cable) in query.iter_mut() {
            if let (Ok(start_transform), Ok(end_transform)) = (
                slots_q.get(cable.start_entity),
                slots_q.get(cable.end_entity),
            ) {
                let start = start_transform.translation.truncate();
                let end = end_transform.translation.truncate();

                let direction = end - start;
                let length = direction.length();
                let angle = direction.y.atan2(direction.x);

                transform.translation = ((start + end) / 2.0).extend(CABLE_Z_INDEX); // Mid
                transform.rotation = Quat::from_rotation_z(angle); // Align with direction
                transform.scale = Vec3::new(length, CABLE_THICKNESS, 1.0); // Adjust size
            }
        }
    }

    /// Spawn a cable preview
    pub fn spawn_preview(
        commands: &mut Commands,
        start_entity: Entity,
    ) {
        commands.spawn((
            PickingBehavior::IGNORE,
            Sprite {
                color: hex!(CABLE_COLOR),
                custom_size: Some(Vec2::new(1.0, CABLE_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, CABLE_Z_INDEX),
            CablePreview { start_entity },
        ));
    }

    /// Update system for cable preview
    pub fn update_previews(
        mut query: Query<(&mut Transform, &mut Sprite, &CablePreview)>,
        slots_q: Query<&GlobalTransform, (With<CableSlot>, Without<Cable>)>,
        windows_q: Query<&Window>,
        camera_q: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    ) {
        let window = windows_q.single();
        let (camera, camera_transform) = camera_q.single();
        let Ok((mut transform, mut sprite, cable)) = query.get_single_mut() else { return };
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            if let Ok(start_transform) = slots_q.get(cable.start_entity) {
                let start = start_transform.translation().truncate();

                let direction = world_position - start;
                let length = direction.length();
                let angle = direction.y.atan2(direction.x);

                if length > MAX_CABLE_LENGTH {
                    sprite.color = hex!("#ff0000");
                } else {
                    sprite.color = hex!(CABLE_COLOR);
                }

                transform.translation = ((start + world_position) / 2.0).extend(CABLE_Z_INDEX); // Mid
                transform.rotation = Quat::from_rotation_z(angle); // Align with direction
                transform.scale = Vec3::new(length, CABLE_THICKNESS, 1.0); // Adjust size
            }
        }
    }

    /// Remove all cable previews
    pub fn remove_previews(commands: &mut Commands, q: Query<Entity, With<CablePreview>>) {
        if let Ok(entity) = q.get_single() {
            commands.entity(entity).despawn();
        }
    }
}
