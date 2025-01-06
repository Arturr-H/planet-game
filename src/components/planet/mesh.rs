use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::PI;
use super::Planet;

pub fn generate_planet_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    radii: &Vec<(f32, f32)>,
) -> Handle<Mesh> {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let resolution = radii.len();
    let mut indices = Vec::new();
    let mut vertices = Vec::new();
    for i in 0..resolution {

        let curr_radius = radii[i].1;
        let next_radius = radii[(i + 1) % resolution].1;
        let (x, y) = (radii[i].0.cos() * curr_radius, radii[i].0.sin() * curr_radius);
        let (nx, ny) = (radii[(i + 1) % resolution].0.cos() * next_radius, radii[(i + 1) % resolution].0.sin() * next_radius);
        vertices.push([x, y, 0.0]);
        vertices.push([nx, ny, 0.0]);
        vertices.push([0.0, 0.0, 0.0]);
        let i = i as u32;
        indices.push(i * 3);
        indices.push(i * 3 + 1);
        indices.push(i * 3 + 2);
    }
    
    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices,
    );

    meshes.add(mesh)
}

#[derive(Component)]
pub struct VeryStupidMesh;

pub fn update_star(
    kb: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &VeryStupidMesh, &Mesh2d)>,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
    // time: Res<Time>,
) -> () {
    if kb.pressed(KeyCode::Space) {
        println!("Space pressed");
        /* Delete star and spawn new one */
        for (entity, _, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }

        // generate_planet_mesh(commands, meshes, materials, time);
    }
}
