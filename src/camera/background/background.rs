/* Imports */
use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}};

use crate::{components::planet::PlanetMaterial, utils::color::hex};

/* Constants */

/// Background component
#[derive(Component)]
pub struct Background;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Component)]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub time: f32,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/space.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/space.wgsl".into()
    }
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Background::setup)
            .add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
            // .add_systems(Update, Background::update);
    }
}

impl Background {
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut background_material: ResMut<Assets<BackgroundMaterial>>,
    ) -> () {

        let positions = vec![
        [-1.0, -1.0, 0.0], 
        [1.0, -1.0, 0.0],  
        [-1.0, 1.0, 0.0],  
        [1.0, 1.0, 0.0],  
        ];

        let uvs = vec![
            [0.0, 0.0], 
            [1.0, 0.0], 
            [0.0, 1.0], 
            [1.0, 1.0],
        ];

        let indices = Indices::U32(vec![
            0, 1, 2, 
            2, 1, 3, 
        ]);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(indices);

        commands.spawn((
            Background,
            Mesh2d(meshes.add(mesh)),
            Transform::from_xyz(0.0, 0.0, -5.0),
                // .with_scale(Vec3::new(1000.0, 1000.0, 1.0)),
            MeshMaterial2d (background_material.add(BackgroundMaterial{ time: 0.0 }))
        ));
    }
    
    // pub fn update(
    //     time: Res<Time>,
    //     mut query: Query<(&Background, &mut BackgroundMaterial)>,
    // ) -> () {
    //     for (_, mut material) in query.iter_mut() {
    //         material.time += time.delta_secs();
    //     }
    // }
}
