/* Imports */
use std::{f32::consts::{PI, TAU}, fmt::Debug};
use bevy::{prelude::*, utils::HashMap};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::cable::Cable, debug::debug::DebugComponent, foliage::{animation::WindSwayPlugin, tree::Tree, Foliage}, tile::{types::landed_rocket::LandedRocket, Tile, TILE_SIZE}}, functional::damageable::Damageable, systems::{game::{GameState, PlanetResources}, traits::GenericTile}, utils::color::hex, RES_HEIGHT, RES_WIDTH};

use super::mesh::{generate_planet_mesh, VeryStupidMesh};

/* Constants */
const PLANET_ROTATION_SPEED: f32 = 170.0;
const FOLIAGE_SPAWNING_CHANCE: f32 = 0.8;
const SEED: u64 = 1239372178378;

#[derive(Component, Clone)]
pub struct Planet {
    id: usize,

    /// All tiles on the planet (id, tile)
    pub tiles: HashMap<usize, Tile>,

    /// The current tile id we're at. Not public because
    /// the `new_tile_id` method handles incrementing and
    /// returning the new id.
    tile_id: usize,

    /// The resources of the planet
    /// TODO: Maybe move this to a player instead?
    pub resources: PlanetResources,

    /// The entity of the planet, used for e.g getting
    /// the center of the planets (transforms) and such.
    pub planet_entity: Option<Entity>,

    /// The radius of the planet, used to calculate
    /// the position of tiles and such.
    pub radius: f32,

    /// The seed of the planet, used to generate the
    /// surface of the planet.
    pub seed: u64,
}

/// This struct is used to mark a planet as the
/// current players (on this device) planet.
/// 
/// So we can query only the players planet
/// via `Query<&Planet, With<PlayerPlanet>>`.
#[derive(Component)]
pub struct PlayerPlanet;

impl Planet {
    // Init
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut game_state: ResMut<GameState>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        // time: Res<Time>,
        asset_server: Res<AssetServer>
    ) -> () {
        let radius: f32 = RES_WIDTH * 0.625;
        // let mut planet_bundle = commands.spawn((
        //     Sprite {
        //         image: asset_server.load("../assets/planet/planet.png"),
        //         custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
        //         ..default()
        //     },
        //     Transform::from_xyz(0.0, -radius * 1.1, 1.0)
        //     .with_rotation(Quat::from_rotation_z(PI / 2.0)),
        //     PIXEL_PERFECT_LAYERS,
        //     PickingBehavior::IGNORE,
        // ));
        let mesh = generate_planet_mesh(&mut meshes, radius, SEED);
        let mut planet_bundle = commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(hex!("#213823"))),
            VeryStupidMesh,
            Transform::from_xyz(0.0, -radius * 1.1, 1.0)
        ));

        /* Insert planet component */
        let planet = Planet {
            id: game_state.new_planet_id(),
            tiles: HashMap::new(),
            tile_id: 0,
            resources: PlanetResources::default(),
            planet_entity: Some(planet_bundle.id()),
            radius,
            seed: SEED,
        };
        planet_bundle.insert(planet.clone());

        // TODO: Only insert if it's the players own
        planet_bundle.insert(PlayerPlanet);

        // let rocket_tile_id = planet.new_tile_id();
        // planet_bundle.with_children(|parent| {
        //     LandedRocket.spawn(
        //         parent, false,
        //         planet.degree_to_transform(0.0, 0.0, 2.0),
        //         &asset_server, rocket_tile_id
        //     );
        // });

        /* Initialize foliage */
        let mut rng = rand::thread_rng();
        for degree in Foliage::generate_foliage_positions(20) {
            let origin_offset = -6.0 - rng.gen_range(0.0..5.0);
            let z = -0.5 - rng.gen_range(-0.1..0.1);
            let transform = planet.degree_to_transform(degree * 180.0 / PI, origin_offset, z);
            let scale = rng.gen_range(0.8..1.3);
            planet_bundle.with_children(|parent| {
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
        planet_q: Query<&Planet, With<PlayerPlanet>>,
    ) -> () {
        let planet = planet_q.single();
        if keyboard_input.pressed(KeyCode::ArrowRight)
        || keyboard_input.pressed(KeyCode::KeyD) {
            query.single_mut().rotate_z(time.delta_secs() * planet.rotation_speed());
        }
        else if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyA) {
            query.single_mut().rotate_z(-time.delta_secs() * planet.rotation_speed());
        }
    }

    /// Returns a vector of all (radii) (multiple radiusses) of 
    /// the planet. 
    /// 
    /// These radii will be placed every radii.len() / 2π radians
    /// I don't really know how to explain it. Think of multiple
    /// poles being placed from the circle origin, with differing
    /// heights, all being placed next to eachother.
    pub fn get_surface_radii(seed: u64, points: usize, radius: f32) -> Vec<f32> {
        /* I think it's one radius many radii but idk */
        let mut radii: Vec<f32> = Vec::new();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let perlin = Perlin::new(rng.gen_range(0..10000));
        // let radius: f32 = 100.0;
        let noise_amplitude: f32 = 5.0;
        let noise_freq: f64 = 0.1251125561; // Needs to be kinda irrational
        
        /* Generate radii */
        for i in 0..points {
            let noise = perlin.get([noise_freq + (i as f64) * noise_freq]);
            radii.push(radius + noise as f32 * (noise_amplitude + rng.gen_range(-0.05..0.05)));
        }

        radii
    }

    /// Ticks every planet
    fn tick(mut planets: Query<&mut Planet>) -> () {
        for mut planet in planets.iter_mut() {
            let keys = planet.tiles.keys().cloned().collect::<Vec<usize>>();
            for key in keys {
                let tile = planet.tiles.get(&key).unwrap();
                if tile.can_distribute_energy() {
                    Tile::distribute_energy(
                        tile.energy_output(),
                        tile.tile_id,
                        &mut planet
                    );
                }
            }
        }
    }

    /// Increments the tile_id and then returns it
    pub fn new_tile_id(&mut self) -> usize {
        let slot_id = self.tile_id;
        self.tile_id += 1;
        slot_id
    }

    /// If two tiles are connected via cables
    pub fn powergrid_tiles_are_connected(&self, a: usize, b: usize) -> bool {
        match self.tiles.get(&a) {
            Some(e) => e.powergrid_status().connected_tiles.contains(&b),
            None => false,
        }
    }
    pub fn powergrid_register_connection(&mut self, a: usize, b: usize) -> () {
        if let Some(e) = self.tiles.get_mut(&a) {
            e.powergrid_status_mut().connected_tiles.push(b);
        }
        if let Some(e) = self.tiles.get_mut(&b) {
            e.powergrid_status_mut().connected_tiles.push(a);
        }
    }

    pub const fn radius(&self) -> f32 { self.radius }
    pub const fn diameter(&self) -> f32 { self.radius * 2.0 }
    pub const fn circumference(&self) -> f32 { self.diameter() * PI }
    pub const fn rotation_speed(&self) -> f32 { PLANET_ROTATION_SPEED / self.radius }

    /// The angular step between two tiles on the planet. Each tile
    /// is placed somewhere on the circumference of the planet, and
    /// the position of the tile is just stored as an angle. This constant
    /// is the angular distance between two tiles.
    pub const fn angular_step(&self) -> f32 { TILE_SIZE / self.radius }
    pub const fn tile_places(&self) -> usize { (TAU / self.angular_step()) as usize }

    /// Get planet entity or panic
    pub fn planet_entity(&self) -> Entity { self.planet_entity.unwrap() }

    /// Returns a transform from a radians on the planet, somwhere on the
    /// circumference of the planet.
    pub fn degree_to_transform(&self, degree: f32, origin_offset: f32, z: f32) -> Transform {
        let x = degree.cos() * (self.radius + origin_offset);
        let y = degree.sin() * (self.radius + origin_offset);
        let rotation = Quat::from_rotation_z(degree - std::f32::consts::PI / 2.0);
        Transform { translation: Vec3::new(x, y, z), rotation, ..default() }
    }

    /// Jag kan inte förklara denna på engelska. Men den ger tillbaka en Vec3
    /// som man kan multiplicera med ett värde, exempelvis 5.0, vilket ger tillbaka
    /// en Vec3 som är 5.0 units längre ifrån origo av planeten.
    /// 
    /// Eftersom om man skulle addera 5.0 på y koordinaten blir det inte rätt för
    /// de flesta vinklarna. 
    pub fn forward(transform: &Transform) -> Vec3 {
        let forward = transform.rotation * Vec3::Y;
        let forward_2d = Vec2::new(forward.x, forward.y).normalize().extend(0.0);
        forward_2d
    }
}

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WindSwayPlugin)
            .add_systems(Startup, Planet::setup)
            .add_systems(Update, Planet::update);
    }
}
