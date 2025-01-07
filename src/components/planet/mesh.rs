use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}, text::cosmic_text::ttf_parser::vorg::VerticalOriginMetrics};
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

    vertices.push([0.0, 0.0, 0.0]);
    uvs.push([0.5, 0.5]);

    // Planar projection
    let max_radius = radii.iter().map(|(_, r)| *r).fold(0.0, f32::max);

    for i in 0..resolution {
        let (curr_angle, curr_radius) = radii[i];
        let x = curr_angle.cos() * curr_radius;
        let y = curr_angle.sin() * curr_radius;
        vertices.push([x, y, 0.0]);

        //& Bevy method
        // let uv = Vec2::from_angle(-curr_angle).mul_add(Vec2::splat(0.5), Vec2::splat(0.5));
        // uvs.push([uv.x, uv.y]);

        //& Planar projection
        /*
        Vi tar den normaliserade positionen
        för att få kordinaterna i intervallet [-1, 1]
        genom att multiplicera med 0.5 [-0.5, 0.5] och 
        addera 0.5 så får vi intervallet [0, 1] (UV space)
        */
        let normalized_pos = Vec2::new(x, y) / max_radius; 
        uvs.push([normalized_pos.x * 0.5 + 0.5, normalized_pos.y * 0.5 + 0.5]);

        /*
        & Cylindrical Projection 
        knas
            let theta = y.atan2(x);
            let r = (x * x + y * y).sqrt() / curr_radius; // Normalized radius
            uvs.push([
                (theta / (2.0 * PI)) + 0.5, // -PI to PI -> 0 to 1
                r
            ]);
         */ 
    }

    for i in 0..resolution {
        let next = (i + 1) % resolution;
        indices.extend_from_slice(&[0, i as u32 + 1, next as u32 + 1]);
    }
    
    mesh.insert_indices(Indices::U32(indices));
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
