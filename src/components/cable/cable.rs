use std::f32::consts::PI;

/* Imports */
use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d, Material2dPlugin}};
use crate::{camera::OuterCamera, utils::color::hex};
use super::slot::CableSlot;

/* Constants */
const CABLE_Z_INDEX: f32 = 3.0;
const CABLE_THICKNESS: f32 = 12.5;
const MAX_HEIGHT_CABLE: f32 = 18.0;
const CABLE_COLOR: &str = "#020410";
pub const MAX_CABLE_LENGTH: f32 = 200.0;

/// Plugin to add cable rendering functionality
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CableMaterial {
    #[uniform(0)]
    pub dimensions: Vec2<>,
    /// 0 if not exceeded, 1 if exceeded, bools are not implemented in ShaderType
    #[uniform(1)]
    pub exceeded_length: u32,
}

impl Material2d for CableMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cable.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub struct CablePlugin;
impl Plugin for CablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<CableMaterial>::default())
            .add_systems(Update, (Cable::update_cables, Cable::update_previews));
    }
}



/// Component to represent a cable between two entities
#[derive(Component)]
pub struct Cable {
    pub start_entity: Entity,
    pub end_entity: Entity,

    pub start_tile_id: usize,
    pub end_tile_id: usize,
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

        start_tile_id: usize,
        end_tile_id: usize,
        mut cable_materials: ResMut<Assets<CableMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            PickingBehavior::IGNORE,
            // Sprite {
            //     color: hex!(CABLE_COLOR),
            //     custom_size: Some(Vec2::new(1.0, 1.0)),
            //     ..default()
            // },
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(cable_materials.add(CableMaterial {dimensions: Vec2::new(1.0, 1.0), exceeded_length: 0})),
            Transform::from_xyz(0.0, 0.0, CABLE_Z_INDEX),
            Cable {
                start_entity,
                end_entity,

                start_tile_id,
                end_tile_id,
            },
        ));
    }

    /// Update system
    fn update_cables(
        mut query: Query<(&mut Transform, &Cable, &mut MeshMaterial2d<CableMaterial>)>,
        slots_q: Query<&Transform, (With<CableSlot>, Without<Cable>)>,
        mut cable_material: ResMut<Assets<CableMaterial>>,
    ) {
        for (mut transform, cable, mesh_material) in query.iter_mut() {
            if let (Ok(start_transform), Ok(end_transform)) = (
                slots_q.get(cable.start_entity),
                slots_q.get(cable.end_entity),
            ) {
                let start = start_transform.translation.truncate();
                let end = end_transform.translation.truncate();

                let start_angle = start.y.atan2(start.x);
                let end_angle = end.y.atan2(end.x);

                // Determine which is further away between start and end to ensure the cable is drawn in the correct direction
                let (left, right) = if (end_angle - start_angle + 2.0 * PI) % (2.0 * PI) < PI {
                    (end, start)
                } else {
                    (start, end)
                };

                let direction = left - right;
                let length = direction.length();
                let height = 1.2 + (MAX_HEIGHT_CABLE - 1.2) * 
                (1.0 - (-length / 80.0).exp());
                let height = height.clamp(1.2, MAX_HEIGHT_CABLE);

                let angle = direction.y.atan2(direction.x);

                let midpoint = (right + left) / 2.0;
                let offset = Vec2::new(-direction.y, direction.x).normalize() * height / 2.0;

                // let aspect_ratio = length / height;
                if let Some(material) = cable_material.get_mut(&mesh_material.0) {
                    material.dimensions = Vec2::new(length, height);
                }

                transform.translation = (midpoint + offset).extend(CABLE_Z_INDEX);
                                        // + Vec3::new((angle + PI / 2.0).cos() * MAX_HEIGHT_CABLE, (angle + PI / 2.0).sin() * MAX_HEIGHT_CABLE, 0.0); // Mid
                transform.rotation = Quat::from_rotation_z(angle); // Align with direction
                transform.scale = Vec3::new(length, height, 1.0); // Adjust size
            }
        }
    }

    /// Spawn a cable preview
    pub fn spawn_preview(
        commands: &mut Commands,
        start_entity: Entity,
        mut cable_materials: ResMut<Assets<CableMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            PickingBehavior::IGNORE,
            // Sprite {
            //     color: hex!(CABLE_COLOR),
            //     custom_size: Some(Vec2::new(1.0, CABLE_THICKNESS)),
            //     ..default()
            // },
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(cable_materials.add(CableMaterial {dimensions: Vec2::new(1.0, 1.0), exceeded_length: 0})),
            Transform::from_xyz(0.0, 0.0, CABLE_Z_INDEX),

            CablePreview { start_entity },
        ));
    }

    /// Update system for cable preview
    pub fn update_previews(
        mut query: Query<(&mut Transform, &mut MeshMaterial2d<CableMaterial>, &CablePreview)>,
        slots_q: Query<&GlobalTransform, (With<CableSlot>, Without<Cable>)>,
        windows_q: Query<&Window>,
        camera_q: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
        mut cable_material: ResMut<Assets<CableMaterial>>,
    ) {
        let window = windows_q.single();
        let (camera, camera_transform) = camera_q.single();
        let Ok((mut transform, mesh_material, cable)) = query.get_single_mut() else { return };
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            if let Ok(start_transform) = slots_q.get(cable.start_entity) {
                let start = start_transform.translation().truncate();

                let start_angle = start.y.atan2(start.x);
                let end_angle = world_position.y.atan2(world_position.x);

                // Determine which is further away between start and end to ensure the cable is drawn in the correct direction
                let (left, right) = if (end_angle - start_angle + 2.0 * PI) % (2.0 * PI) < PI {
                    (world_position, start)
                } else {
                    (start, world_position)
                };

                let direction = left - right;
                let length = direction.length();
                let height = 1.2 + (MAX_HEIGHT_CABLE - 1.2) * 
                (1.0 - (-length / 80.0).exp());
                let height = height.clamp(1.2, MAX_HEIGHT_CABLE);

                let angle = direction.y.atan2(direction.x);

                let midpoint = (right + left) / 2.0;
                let offset = Vec2::new(-direction.y, direction.x).normalize() * height / 2.0;

                // if length > MAX_CABLE_LENGTH {
                //     // sprite.color = hex!("#ff0000");
                // } else {
                //     // sprite.color = hex!(CABLE_COLOR);
                // }

                if let Some(material) = cable_material.get_mut(&mesh_material.0) {
                    material.dimensions = Vec2::new(length, height);

                    if length > MAX_CABLE_LENGTH {
                        material.exceeded_length = 1;
                    } else {
                        material.exceeded_length = 0;
                    }
                }

                transform.translation = (midpoint + offset).extend(CABLE_Z_INDEX);
                transform.rotation = Quat::from_rotation_z(angle); // Align with direction
                transform.scale = Vec3::new(length, height, 1.0); // Adjust size
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
