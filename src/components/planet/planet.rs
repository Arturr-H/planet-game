/* Imports */
use std::{f32::consts::{PI, TAU}, fmt::Debug};
use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d, Material2dPlugin}, utils::HashMap};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::{components::{foliage::{animation::WindSwayPlugin, grass::Grass, stone::Stone, tree::Tree, Foliage}, tile::{Tile, TILE_SIZE}}, systems::game::{GameState, PlanetResources}, utils::color::hex, RES_WIDTH};
use super::mesh::{generate_planet_mesh, VeryStupidMesh};

/* Constants */
const PLANET_ROTATION_SPEED: f32 = 500.0;
const FOLIAGE_SPAWNING_CHANCE: f32 = 0.8;
const SURFACE_RESOLUTION: usize = 100; // How many different vertices for the suface

const PLANET_SHADER_PATH: &str = "shaders/test.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PlanetMaterial {
    #[uniform(0)]
    color1: LinearRgba,
    #[uniform(1)]
    color2: LinearRgba,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        PLANET_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}


#[derive(Component, Clone)]
pub struct Planet {
    id: usize,

    /// All tiles on the planet. Tiles are most often
    /// things that the player places. E.g solar panels,
    /// power poles, drills etc.
    pub tiles: HashMap<usize, Tile>,

    /// Planet POI:s are often things that are generated
    /// on the planet. E.g stones that can be mined.
    pub points_of_interest: HashMap<usize, PlanetPointOfInterest>,

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
    pub seed: u32,

    /// The planets radii
    /// Vec<(angle, radius or height)>
    pub radii: Vec<(f32, f32)>,
}

/// Something that can be interacted with other machines
#[derive(Component, Clone, Debug)]
pub enum PlanetPointOfInterest {
    Stone
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
        mut planet_materials: ResMut<Assets<PlanetMaterial>>,
        asset_server: Res<AssetServer>
    ) -> () {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen_range(0..100_000_000);
        let radius: f32 = RES_WIDTH * 0.625;

        /* Spawn mesh & other things */
        let radii = Planet::get_surface_radii(seed, SURFACE_RESOLUTION, radius);
        let mesh = generate_planet_mesh(&mut meshes, &radii);
        let mut planet_bundle = commands.spawn((
            // Mesh2d(meshes.add(Circle::new(radius))),
            Mesh2d(mesh),
            MeshMaterial2d(planet_materials.add(PlanetMaterial {
                color1: LinearRgba::rgb(1.0, 0.41, 0.71),
                color2: LinearRgba::rgb(0.8, 1.0, 0.0),
            })),
            VeryStupidMesh,
            PickingBehavior::IGNORE,
            // AlphaMode2d::Blend,
            Transform::from_xyz(0.0, -radius * 1.1, 1.0),
        ));
        // planet_bundle.with_children(|parent| {
        //     Self::generate_water(radius, parent, &mut meshes, &mut materials);
        // });

        /* Insert the Planet component */
        let mut planet = Self {
            id: game_state.new_planet_id(),
            points_of_interest: HashMap::new(),
            tiles: HashMap::new(),
            tile_id: 0,
            resources: PlanetResources::default(),
            planet_entity: Some(planet_bundle.id()),
            radius,
            radii,
            seed,
        };
        planet_bundle.insert(PlayerPlanet); // TODO: Only insert if it's the players own
        // planet_bundle.with_children(|parent| {
        //     DebugComponent::setup(parent, "0*pi rad (right)", planet.radians_to_transform(0.0, 0.0, 5.0));
        //     DebugComponent::setup(parent, "pi/4 rad", planet.radians_to_transform(PI / 4.0, 0.0, 5.0));
        //     DebugComponent::setup(parent, "pi/2 rad", planet.radians_to_transform(PI / 2.0, 0.0, 5.0));
        // });

        /* Initialize foliage */
        let mut rng = rand::thread_rng();
        for degree in Foliage::generate_foliage_positions(0.8, seed) {
            let origin_offset = -6.0 - rng.gen_range(0.0..5.0);
            let z = -0.5 - rng.gen_range(-0.1..0.1);
            let transform = planet.radians_to_transform(degree, origin_offset, z);
            let scale = rng.gen_range(0.8..1.3);
            planet_bundle.with_children(|parent| {
                Tree::spawn(
                    parent,
                    &asset_server,
                    transform.with_scale(Vec3::splat(scale))
                );
            });
        }
        for degree in Foliage::generate_foliage_positions(0.8, seed) {
            let origin_offset = -6.0 - rng.gen_range(0.0..5.0);
            let z = -0.5 - rng.gen_range(-0.1..0.1);
            let transform = planet.radians_to_transform(degree, origin_offset, z);
            let scale = rng.gen_range(0.8..1.3);
            planet_bundle.with_children(|parent| {
                Grass::spawn(
                    parent,
                    &asset_server,
                    transform.with_scale(Vec3::splat(scale))
                );
            });
        }

        let origin_offset = -10.0 - rng.gen_range(0.0..5.0);
        let z = -0.5 - rng.gen_range(-0.1..0.1);
        let index = 4;
        let transform = planet.index_to_transform(index, origin_offset, z);
        planet.points_of_interest.insert(index, PlanetPointOfInterest::Stone);
        planet_bundle.with_children(|parent| {
            Stone::spawn(
                parent,
                &asset_server,
                transform
            );
        });

        planet_bundle.insert(planet.clone());
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
    /// 
    /// Returns Vec<(angle, radius)>
    pub fn get_surface_radii(seed: u32, resolution: usize, radius: f32) -> Vec<(f32, f32)> {
        /* I think it's one radius many radii but idk */
        let mut radii: Vec<(f32, f32)> = Vec::with_capacity(resolution);
        let noise_freq: f64 = 0.1251125561; // Needs to be kinda irrational
        let perlin = Perlin::new(seed);
        let noise_amplitude: f32 = 10.0;

        /* Generate radii */
        for i in 0..resolution {
            let noise = perlin.get([noise_freq + (i as f64) * noise_freq]) as f32;
            let mut height = radius + noise * noise_amplitude;
            let angle = (PI * 2.0 / resolution as f32) * i as f32;

            // These stupid if else statements are needed because
            // we generate a noise vector that it NOT connected from
            // the last to the first noise. Say we have vector of values
            // symbolizing a wave. vec![4, 5, 4, 1, -1, -2]. If we
            // repeat that noise value (our planet is circular) we have a
            // big jump from -2 to 4. These if else statements just tries
            // to smooth it out a bit. Not perfect though.
            if i == resolution - 1 {
                height = (radii[0].1 + height) / 2.0;
            }

            radii.push((angle, height));
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

    fn generate_water(
        radius: f32,
        commands: &mut ChildBuilder,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> () {
        let circle = Mesh::from(Circle::new(radius));

        commands.spawn((
            Mesh2d(meshes.add(circle)),
            MeshMaterial2d(materials.add(hex!("#003080"))),
            Transform::from_xyz(0.0, 0.0, -1.),
        ));

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

    /// If number lies within the radius of other_number, also wraps around
    /// the function `tile_places`.
    /// 
    /// E.g if the number = 99, and the amount of tile_places on the planet
    /// is 100, and the other_number = 1, and the radius = 2, then the function
    /// will return true because the distance between 99 and 1 is 2.
    /// 
    /// This function is used to calculate if e.g a drill can drill a stone POI
    /// (if the stone is close enough to the drill).
    pub const fn number_in_radius(&self, number: usize, other_number: usize, radius: usize) -> bool {
        let clockwise_distance = (number + self.tile_places() - other_number) % self.tile_places();
        let counterclockwise_distance = (other_number + self.tile_places() - number) % self.tile_places();
        clockwise_distance <= radius || counterclockwise_distance <= radius
    }

    /// Get planet entity or panic
    pub fn planet_entity(&self) -> Entity { self.planet_entity.unwrap() }

    /// Returns a transform from a radians on the planet, somwhere on the
    /// circumference of the planet.
    /// 
    /// ## WARNING
    /// `radians` needs to be between 0..2π
    pub fn radians_to_transform(&self, radians: f32, origin_offset: f32, z: f32) -> Transform {
        /* Where we are around the world */
        let radians_normalized = (radians / self.angular_step()).round() / self.tile_places() as f32;
        let radii_index = SURFACE_RESOLUTION as f32 * radians_normalized;
        let radii_index_int = (SURFACE_RESOLUTION as f32 * radians_normalized)
            .min(SURFACE_RESOLUTION as f32 - 1.0) as usize;
        let radii_index_decimals = radii_index - radii_index_int as f32; // 0.0-1.0

        let (curr_angle, curr_height) = self.radii[radii_index_int];
        let (next_angle, next_height) = self.radii[(radii_index_int + 1) % self.radii.len()];
        let curr_amp = curr_height + origin_offset;
        let next_amp = next_height + origin_offset;
        let point_a = Vec2::new(curr_angle.cos() * curr_amp, curr_angle.sin() * curr_amp);
        let point_b = Vec2::new(next_angle.cos() * next_amp, next_angle.sin() * next_amp);
        let delta = point_b - point_a;
        let new = point_a + delta * radii_index_decimals;

        /* Surface rotation */
        let dy = point_b.y - point_a.y;
        let dx = point_b.x - point_a.x;
        let surface_radians = dy.atan2(dx);

        let rotation = Quat::from_rotation_z(surface_radians + PI);
        Transform { translation: Vec3::new(new.x, new.y, z), rotation, ..default() }
    }
    pub fn index_to_transform(&self, index: usize, origin_offset: f32, z: f32) -> Transform {
        assert!(index < self.tile_places(), "Index needs to be less than the amount of tile places on the planet");
        let radians = index as f32 * self.angular_step();
        self.radians_to_transform(radians, origin_offset, z)
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
            .add_plugins((
                WindSwayPlugin,
                Material2dPlugin::<PlanetMaterial>::default(),
            ))
            .add_systems(Startup, Planet::setup)
            .add_systems(Update, Planet::update)
            .add_systems(FixedUpdate, Planet::tick);
    }
}
