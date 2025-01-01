use std::{f32::consts::PI, fmt::Debug};

/* Imports */
use bevy::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::cable::Cable, foliage::{foliage::{Foliage, FoliagePlugin}, texture::FoliageTextures}, slot::Slot, tile::Tile}, functional::damageable::Damageable, systems::game::GameState, PLANET_SLOTS, RES_HEIGHT, RES_WIDTH};

/* Constants */
const PLANET_ROTATION_SPEED: f32 = 1.7;
const FOLIAGE_SPAWNING_CHANCE: f32 = 0.8;

#[derive(Component)]
pub struct Planet;
pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FoliagePlugin)
            .add_systems(Startup, setup.after(FoliageTextures::load_textures))
            .add_systems(Update, update);
    }
}

// Init
fn setup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
    foliage_textures: Res<FoliageTextures>
) -> () {
    let mut planet = commands.spawn((
        /* Round */
        Sprite {
            image: asset_server.load("../assets/planet/planet.png"),
            custom_size: Some(Vec2::new(RES_WIDTH, RES_WIDTH)),
            ..default()
        },
        Transform::from_xyz(0.0, -(RES_WIDTH / 2.0), 1.0),
        PIXEL_PERFECT_LAYERS,
        PickingBehavior::IGNORE,
        Planet,
    ));

    /* Initialize slots */
    const DEGREE_STEP: f32 = 360.0 / PLANET_SLOTS as f32;
    for slot_id in 0..PLANET_SLOTS {
        let degree = (slot_id as f32 * DEGREE_STEP).to_radians();
        let x = degree.cos() * RES_WIDTH / 2.0;
        let y = degree.sin() * RES_WIDTH / 2.0;
        let rotation = Quat::from_rotation_z(degree - std::f32::consts::PI / 2.0);

        /* Add slot as child of the planet */
        planet.with_children(|parent| {
            Slot::spawn(
                Tile::random(&mut game_state, slot_id),
                parent, &asset_server, &mut game_state, slot_id, Transform {
                    translation: Vec3::new(x, y, 1.0), rotation,
                    ..default()
                }
            );
        });
    }


    /* Initialize foliage */
    let mut degree = 0.0;
    let mut rng = rand::thread_rng();
    let approx_step = PI / 14.0;
    const MIN_SPACE: f32 = 0.1;
    loop {
        if degree > 2.0 * PI { break; }
        degree = rng.gen_range((degree + MIN_SPACE)..(degree + approx_step + MIN_SPACE));
        planet.with_children(|parent| {
            Foliage::spawn(parent, &foliage_textures, degree);
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
