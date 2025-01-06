use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::PI;

use super::Planet;

pub fn generate_planet_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>
) {
    let points = 40;
    let radii = Planet::get_surface_radii((time.elapsed_secs() * 100.0) as u64, points);
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let mut indices = Vec::new();
    let mut vertices = Vec::new();
    for i in 0..points {
        let curr_radius;
        let next_radius;
        if i == points - 2 {
            curr_radius = radii[i];
            next_radius = (radii[points - 1] + radii[0]) / 2.0;
        }else if i == points - 1 {
            curr_radius = (radii[points - 1] + radii[0]) / 2.0;
            next_radius = radii[0];
        }else {
            curr_radius = radii[i];
            next_radius = radii[i + 1];
        }
        let angle = (PI * 2.0 / points as f32) * i as f32;
        let next_angle = PI * 2.0 / (points as f32) * ((i + 1) as f32);
        let (x, y) = (angle.cos() * curr_radius, angle.sin() * curr_radius);
        let (nx, ny) = (next_angle.cos() * next_radius, next_angle.sin() * next_radius);
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

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        VeryStupidMesh,
    ));
}

#[derive(Component)]
pub struct VeryStupidMesh;

pub fn update_star(
    kb: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &VeryStupidMesh, &Mesh2d)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) -> () {
    if kb.pressed(KeyCode::Space) {
        println!("Space pressed");
        /* Delete star and spawn new one */
        for (entity, _, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }

        generate_planet_mesh(commands, meshes, materials, time);
    }
}
