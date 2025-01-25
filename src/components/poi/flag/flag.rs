/* Imports */
use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::{Anchor, Material2dPlugin}};
use crate::components::planet::Planet;
use super::material::{FlagMaterial, FlagMaterialParams};

/* Constants */
const FLAG_HEIGHT: f32 = 15.0;
const FLAG_WIDTH: f32 = 22.5;

/// Flag component
#[derive(Component)]
pub struct Flag;

pub struct SpawnFlag {
    pub transform: Transform,
}
impl EntityCommand for SpawnFlag {
    fn apply(self, entity: Entity, world: &mut World) {
        Flag::setup(self, entity, world);
    }
}

impl Flag {
    pub fn setup(
        flag: SpawnFlag,
        entity: Entity,
        world: &mut World,
    ) {
        // Create the mesh
        let mesh = Self::create_flag_mesh(FLAG_WIDTH, FLAG_HEIGHT);

        // Temporarily borrow `world` to get the resources
        let mesh_handle = {
            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            meshes.add(mesh)
        };

        let material_handle = {
            let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
            materials.add(ColorMaterial {
                color: Color::srgb(0.0, 0.0, 1.0),
                ..Default::default()
            }/*FlagMaterial {
                p
            }*/)
        };

        let image_handle = {
            let images = world.resource_mut::<AssetServer>();
            images.load("planet/flag-pole.png")
        };

        // Spawn the entity with the mesh and material
        world.commands().entity(entity).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: image_handle,
                    anchor: Anchor::BottomCenter,
                    ..Default::default()
                },
                flag.transform,
            )).with_child((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::from_xyz(FLAG_WIDTH / 2.0 + 1.0, 64.0 - FLAG_HEIGHT / 2.0 - 2.0, -0.1)
                    .with_rotation(Quat::from_rotation_z(-0.07))
            ));
        });
    }
    pub fn update(time: Res<Time>, mut materials: ResMut<Assets<FlagMaterial>>) {
        for material in materials.iter_mut() {
            // material.1.params.time = time.elapsed_secs();
        }
    }
    fn create_flag_mesh(width: f32, height: f32) -> Mesh {
        let mut mesh = Mesh::from(Rectangle::new(width, height));
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 1.0],
            ]),
        );
        mesh
    }
}

pub struct FlagPlugin;
impl Plugin for FlagPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<FlagMaterial>::default())
            .add_systems(Update, Flag::update);
    }
}
