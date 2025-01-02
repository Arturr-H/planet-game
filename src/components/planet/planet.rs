/* Imports */
use std::{f32::consts::PI, fmt::Debug};
use bevy::prelude::*;
use rand::Rng;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::cable::Cable, foliage::{animation::WindSwayPlugin, foliage::Foliage, tree::Tree}, cable::slot::Slot, tile::Tile}, functional::damageable::Damageable, systems::game::GameState, PLANET_SLOTS, RES_HEIGHT, RES_WIDTH};

/* Constants */
const PLANET_ROTATION_SPEED: f32 = 1.7;
const FOLIAGE_SPAWNING_CHANCE: f32 = 0.8;
const PLANET_SIZE: f32 = RES_WIDTH * 1.25;

#[derive(Component)]
pub struct Planet;
pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WindSwayPlugin)
            .add_systems(Startup, Planet::setup)
            .add_systems(Update, Planet::update);
    }
}

impl Planet {
    // Init
    fn setup(
        mut commands: Commands,
        mut game_state: ResMut<GameState>,
        asset_server: Res<AssetServer>
    ) -> () {
        let mut planet = commands.spawn((
            Sprite {
                image: asset_server.load("../assets/planet/planet.png"),
                custom_size: Some(Vec2::new(PLANET_SIZE, PLANET_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, -(PLANET_SIZE / 2.0) * 1.1, 1.0)
            .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            PIXEL_PERFECT_LAYERS,
            PickingBehavior::IGNORE,
            Planet,
        ));

        // /* Initialize slots */
        // const DEGREE_STEP: f32 = 360.0 / PLANET_SLOTS as f32;
        // for slot_id in 0..PLANET_SLOTS {
        //     let degree = (slot_id as f32 * DEGREE_STEP).to_radians();
        //     let transform = Self::degree_to_transform(degree, 5.0, 1.0);

        //     /* Add slot as child of the planet */
        //     planet.with_children(|parent| {
        //         Slot::spawn(
        //             Tile::random(&mut game_state, slot_id),
        //             parent, &asset_server, &mut game_state,
        //             slot_id, transform
        //         );
        //     });
        // }

        /* Initialize foliage */
        let mut rng = rand::thread_rng();
        for degree in Foliage::generate_foliage_positions(20) {
            let origin_offset = -6.0 - rng.gen_range(0.0..5.0);
            let z = -0.5 - rng.gen_range(-0.1..0.1);
            let transform = Self::degree_to_transform(degree * 180.0 / PI, origin_offset, z);
            let scale = rng.gen_range(0.8..1.3);
            planet.with_children(|parent| {
                Tree::spawn(
                    parent,
                    &asset_server,
                    transform.with_scale(Vec3::splat(scale))
                );
            });
        }
    }

    // Update
    fn update(
        time: Res<Time>,
        mut query: Query<&mut Transform, With<Planet>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) -> () {
        if keyboard_input.pressed(KeyCode::ArrowRight)
        || keyboard_input.pressed(KeyCode::KeyD) {
            query.single_mut().rotate_z(time.delta_secs() * PLANET_ROTATION_SPEED);
        }
        else if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyA) {
            query.single_mut().rotate_z(-time.delta_secs() * PLANET_ROTATION_SPEED);
        }
    }

    // Helper
    fn degree_to_transform(degree: f32, origin_offset: f32, z: f32) -> Transform {
        let x = degree.cos() * (PLANET_SIZE / 2.0 + origin_offset);
        let y = degree.sin() * (PLANET_SIZE / 2.0 + origin_offset);
        let rotation = Quat::from_rotation_z(degree - std::f32::consts::PI / 2.0);
        Transform { translation: Vec3::new(x, y, z), rotation, ..default() }
    }
}
