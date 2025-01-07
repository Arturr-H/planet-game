use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::PI;
use super::{planet, Planet};

pub fn generate_planet_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    radii: &Vec<(f32, f32)>,
    arc_length: f32,
) -> Handle<Mesh> {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let resolution = radii.len();
    let mut indices = Vec::new();
    let mut vertices = Vec::new();
    let mut uvs = Vec::new();

    // uvs.push([0.5, 0.5]);

    for i in 0..resolution {

        let curr_radius = radii[i].1;
        let next_radius = radii[(i + 1) % resolution].1;
        let curr_angle = radii[i].0;
        let next_angle = radii[(i + 1) % resolution].0;

        let u = curr_angle / (2.0 * PI);
        let next_u = next_angle / (2.0 * PI);

        let next_u = if next_u < u { next_u + 1.0 } else { next_u };
        // let u = current_length / total_circumference;
        // let next_u = (current_length + arc_length) / total_circumference;
        uvs.push([u, 1.0]);
        uvs.push([next_u, 1.0]);
        uvs.push([u, 0.0]);

                //current_length += arc_length;

        let (x, y) = (curr_angle.cos() * curr_radius, curr_angle.sin() * curr_radius);
        let (nx, ny) = (next_angle.cos() * next_radius, next_angle.sin() * next_radius);
        vertices.push([x, y, 0.0]);
        vertices.push([nx, ny, 0.0]);
        vertices.push([0.0, 0.0, 0.0]);

        //uv
        

        let i = i as u32;
        indices.push(i * 3);
        indices.push(i * 3 + 1);
        indices.push(i * 3 + 2);
    }
    
    mesh.insert_indices(Indices::U32(indices));
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; vertices.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    
    let normals: Vec<[f32; 3]> = vec![[0., 0., 1.]; vertices.len()];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
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
