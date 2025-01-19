/* Imports */
use std::{f32::consts::{PI, TAU}, fmt::Debug};
use bevy::{prelude::*, render::{camera, render_resource::{AsBindGroup, ShaderRef}}, sprite::{AlphaMode2d, Material2d, Material2dPlugin}, utils::HashMap};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::{camera::{post_processing::PostProcessSettings, CameraPlugin, OuterCamera}, components::{foliage::{grass::Grass, rock::Rock, Foliage}, poi::{self, stone::Stone, tree::Tree, PointOfInterest, PointOfInterestType}, tile::{Tile, TileType, TILE_SIZE}}, systems::{game::{GameState, PlanetResources}, traits::{GenericPointOfInterest, GenericTile}}, utils::color::hex, RES_WIDTH};
use super::{debug::{self, PlanetConfiguration}, mesh::generate_planet_mesh};

/* Constants */
const PLANET_ROTATION_SPEED: f32 = 1.5;
const FOLIAGE_SPAWNING_CHANCE: f32 = 0.8;
const PLANET_SHADER_PATH: &str = "shaders/planet.wgsl";
const PLANET_ATMOSPHERE_SHADER_PATH: &str = "shaders/planet_atmosphere.wgsl";
const CAMERA_ELEVATION: f32 = 50.0;
const CAMERA_DAMPING: f32 = 1.0; // 1 = no damping 2 = pretty smooth, less than 1 = do not

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(0)]
    seed: f32,
    #[uniform(1)]
    radius: f32,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        PLANET_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetAtmosphereMaterial {
    #[uniform(0)]
    planet_radius: f32,

    #[uniform(1)]
    zoom: f32,
}

impl Material2d for PlanetAtmosphereMaterial {
    fn fragment_shader() -> ShaderRef {
        PLANET_ATMOSPHERE_SHADER_PATH.into()
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
    pub points_of_interest: HashMap<usize, Vec<PointOfInterest>>,

    /// The resources of the planet
    /// TODO: Maybe move this to a player instead?
    pub resources: PlanetResources,

    /// The entity of the planet, used for e.g getting
    /// the center of the planets (transforms) and such.
    /// TODO: NO OPTION _  TEMP ENTITY INSTEAD
    pub planet_entity: Option<Entity>,

    /// The radius of the planet, used to calculate
    /// the position of tiles and such.
    pub radius: f32,

    /// The seed of the planet, used to generate the
    /// surface of the planet.
    pub seed: u32,
    pub amplitude: f32,
    pub frequency: f64,
    pub resolution: usize,

    /// The planets radii
    /// Vec<(angle, radius or height)>
    pub radii: Vec<(f32, f32)>,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            id: 0,
            points_of_interest: HashMap::new(),
            tiles: HashMap::new(),
            resources: PlanetResources::default(),
            planet_entity: None,
            amplitude: 2000.0,
            frequency: 80.0,
            resolution: 500,
            radius: 1400.0,
            radii: Vec::new(),
            seed: 0,
        }
    }
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
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut game_state: ResMut<GameState>,
        mut planet_materials: ResMut<Assets<PlanetMaterial>>,
        mut planet_atmosphere_materials: ResMut<Assets<PlanetAtmosphereMaterial>>,
        mut camera_q: Query<&mut Transform, With<OuterCamera>>,
        config: ResMut<PlanetConfiguration>,
        asset_server: Res<AssetServer>
    ) -> () {
        let radius = config.radius.max(15.0);
        let seed = config.seed;
        game_state.set_game_seed(seed as u64);

        /* Spawn mesh & other things */
        let radii = Planet::get_surface_radii(&config);
        let mesh = generate_planet_mesh(&mut meshes, &radii);
        let mut planet_bundle = commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(planet_materials.add(PlanetMaterial {
                seed: config.seed as f32,
                radius: config.radius,
            })),
            PickingBehavior::IGNORE,
            Transform::from_xyz(0.0, 0.0, 1.0),
            Name::new("Planet"),
        ));

        planet_bundle.with_children(|parent| {
            parent.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(planet_atmosphere_materials.add(PlanetAtmosphereMaterial {
                    planet_radius: config.radius as f32,
                    zoom: 1.0,
                })),
                Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat((radius + 800.0) / radius)),
            ));
        });

        /* Insert the Planet component */
        let mut planet = Self {
            id: game_state.new_planet_id(),
            points_of_interest: HashMap::new(),
            tiles: HashMap::new(),
            resources: PlanetResources::default(),
            planet_entity: Some(planet_bundle.id()),
            amplitude: config.amplitude,
            frequency: config.frequency,
            resolution: config.resolution,
            radius,
            radii,
            seed,
        };
        planet_bundle.insert(PlayerPlanet); // TODO: Only insert if it's the players own

        /* Initialize foliage */
        let points = (planet.radius / 1.0) as usize;
        planet_bundle.with_children(|parent| {
            Foliage::generate_foliage_positions(
                0.8, 0.5, points, seed,
                Grass::spawn, &asset_server, parent,
                &planet, -1.0
            );
            Foliage::generate_foliage_positions(
                0.6, 0.2, points, seed + 1,
                Rock::spawn, &asset_server, parent,
                &planet, -1.0
            );
        });

        /* Landed rocket */
        // planet_bundle.with_children(|parent| {
        //     let entity = TileType::LandedRocket(LandedRocket).spawn(
        //         parent, false,
        //         planet.index_to_transform(0, 0.0, 5.0),
        //         &asset_server, &mut texture_atlas_layouts, 0
        //     );

        //     planet.tiles.insert(0, Tile::new(0, TileType::LandedRocket(LandedRocket), entity));
        // });

        /* Initialize POI:s */
        planet_bundle.with_children(|parent| {
            planet.generate_pois(parent, &asset_server);
        });
        
        planet_bundle.insert(planet.clone());
        match camera_q.get_single_mut() {
            Ok(mut transform) => {
                Self::update_camera_transform(&planet, 0.0, &mut transform);
            },
            Err(_) => (),
        };

        /* Atmosphere */
        
    }

    // Update
    fn update(
        time: Res<Time>,
        mut camera_q: Query<(&mut Transform, &OrthographicProjection), With<OuterCamera>>,
        mut camera_rotation: ResMut<CameraPlanetRotation>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
        mut planet_atmosphere_materials: ResMut<Assets<PlanetAtmosphereMaterial>>,
    ) -> () {
        let planet = planet_q.single();
        if let Ok((mut camera_transform, projection)) = camera_q.get_single_mut() {
            let mut update = false;
            if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::KeyD) {
                camera_rotation.radians -= (time.delta_secs() * PLANET_ROTATION_SPEED * projection.scale / 3.0).max(0.001).min(0.018);
                update = true;
            }
            else if keyboard_input.pressed(KeyCode::ArrowLeft)
                || keyboard_input.pressed(KeyCode::KeyA) {
                    camera_rotation.radians += (time.delta_secs() * PLANET_ROTATION_SPEED * projection.scale / 3.0).max(0.001).min(0.018);
                update = true;
            }

            planet_atmosphere_materials.iter_mut().for_each(|(_, mat)| {
                mat.zoom = projection.scale;
            });

            if update {
                Self::update_camera_transform(&planet, camera_rotation.radians, &mut camera_transform);
            }
        }
    }
    fn update_camera_transform(planet: &Planet, radians: f32, camera_transform: &mut Transform) -> () {
        let camera_radians = Self::normalize_radians(radians + PI / 2.0);
        let (translation, surface_angle) = planet.radians_to_radii(camera_radians, CAMERA_ELEVATION);
        let mul = (CAMERA_DAMPING - 1.0) * (planet.radius + CAMERA_ELEVATION);
        camera_transform.translation = Vec3::new(
            (translation.x + mul * camera_radians.cos()) / CAMERA_DAMPING,
            (translation.y + mul * camera_radians.sin()) / CAMERA_DAMPING,
            camera_transform.translation.z
        );
        camera_transform.rotation = Quat::from_rotation_z(Self::normalize_radians(surface_angle + PI));
    }

    // | vv ------- PLANET SURFACE MESH ------- vv | \\
    // | vv ------- PLANET SURFACE MESH ------- vv | \\

    /// Returns a vector of all (radii) (multiple radiusses) of 
    /// the planet. 
    /// 
    /// These radii will be placed every radii.len() / 2π radians
    /// I don't really know how to explain it. Think of multiple
    /// poles being placed from the circle origin, with differing
    /// heights, all being placed next to eachother.
    /// 
    /// Returns Vec<(angle, radius)>
    pub fn get_surface_radii(config: &PlanetConfiguration) -> Vec<(f32, f32)> {
        let mut radii: Vec<(f32, f32)> = Vec::with_capacity(config.resolution);
        let perlin = Perlin::new(config.seed);
        let frequency = config.frequency / 100.0;
        let amplitude = config.amplitude / 10.0;
    
        for i in 0..config.resolution {
            let angle = (std::f32::consts::PI * 2.0 / config.resolution as f32) * i as f32;
            let x = angle.cos() as f64 * frequency;
            let y = angle.sin() as f64 * frequency;
            
            let noise = perlin.get([x, y]) as f32;
            let height = noise * amplitude + config.radius;
            println!("{height}");
    
            radii.push((angle, height));
        }
    
        radii
    }
    /// This function is used to get the position of a certain
    /// point on the planet surface from just one radians value.
    /// 
    /// Returns (radii (elevation position from center), angle (slope
    /// of current point))
    fn radians_to_radii(&self, radians: f32, origin_offset: f32) -> (Vec2, f32) {
        let radians = radians % TAU;
        let radians_normalized = (Self::normalize_radians(radians) / self.angular_step()) / self.tile_places() as f32;
        let radii_index = self.resolution() as f32 * radians_normalized;
        let radii_index_int = (self.resolution() as f32 * radians_normalized)
            .min(self.resolution() as f32 - 1.0) as usize;
        let radii_index_decimals = radii_index - radii_index_int as f32; // 0.0-1.0

        let (prev_angle, prev_height) = self.radii[radii_index_int.checked_sub(1).unwrap_or(self.radii.len() - 1)];
        let (curr_angle, curr_height) = self.radii[radii_index_int];
        let (next_angle, next_height) = self.radii[(radii_index_int + 1) % self.radii.len()];
        
        let prev_amp = prev_height + origin_offset;
        let curr_amp = curr_height + origin_offset;
        let next_amp = next_height + origin_offset;

        let point_prev = Vec2::new(prev_angle.cos() * prev_amp, prev_angle.sin() * prev_amp);
        let point_a = Vec2::new(curr_angle.cos() * curr_amp, curr_angle.sin() * curr_amp);
        let point_b = Vec2::new(next_angle.cos() * next_amp, next_angle.sin() * next_amp);
        // let delta_prev = point_a - point_prev;
        let delta = point_b - point_a;
        
        let new = point_a + delta * radii_index_decimals;
        let dy_prev = point_a.y - point_prev.y;
        let dx_prev = point_a.x - point_prev.x;

        let dy = point_b.y - point_a.y;
        let dx = point_b.x - point_a.x;

        let prev_surface_radians = Self::normalize_radians(dy_prev.atan2(dx_prev));
        let surface_radians = Self::normalize_radians(dy.atan2(dx));

        let mut delta_radians = surface_radians - prev_surface_radians;
        if delta_radians > TAU / 2.0 {
            delta_radians -= TAU;
        } else if delta_radians < -TAU / 2.0 {
            delta_radians += TAU;
        }

        let mut new_surface_radians = prev_surface_radians + delta_radians * radii_index_decimals;
        new_surface_radians %= TAU;
        if new_surface_radians < 0.0 {
            new_surface_radians += TAU;
        }
            
        (new, new_surface_radians)
    }

    // | ^^ ------- PLANET SURFACE MESH ------- ^^ | \\
    // | ^^ ------- PLANET SURFACE MESH ------- ^^ | \\

    // --------------------------------------------------

    // | vv ------- CONSTANT GETTERS ------- vv | \\
    // | vv ------- CONSTANT GETTERS ------- vv | \\

    pub const fn radius(&self) -> f32 { self.radius }
    pub const fn diameter(&self) -> f32 { self.radius * 2.0 }
    pub const fn circumference(&self) -> f32 { self.diameter() * PI }
    pub const fn rotation_speed(&self) -> f32 { PLANET_ROTATION_SPEED / self.radius }
    pub const fn resolution(&self) -> usize { self.resolution }
    pub const fn planet_entity(&self) -> Entity { self.planet_entity.unwrap() }

    /// The angular step between two tiles on the planet. Each tile
    /// is placed somewhere on the circumference of the planet, and
    /// the position of the tile is just stored as an angle. This constant
    /// is the angular distance between two tiles.
    pub const fn angular_step(&self) -> f32 { TILE_SIZE / self.radius }
    pub const fn tile_places(&self) -> usize { (TAU / self.angular_step()) as usize }

    // | ^^ ------- CONSTANT GETTERS ------- ^^ | \\
    // | ^^ ------- CONSTANT GETTERS ------- ^^ | \\

    // --------------------------------------------------

    // | vv ------- RADIANS & RADIUS RELATED ------- vv | \\
    // | vv ------- RADIANS & RADIUS RELATED ------- vv | \\

    /// Returns a transform from a radians on the planet, somwhere on the
    /// circumference of the planet.
    /// 
    /// ## WARNING
    /// `radians` needs to be between 0..2π
    pub fn radians_to_transform(&self, radians: f32, origin_offset: f32, z: f32) -> Transform {
        /* What the elevation is at the current angle */
        let (new, surface_radians) = self.radians_to_radii(radians, origin_offset);

        let rotation = Quat::from_rotation_z(surface_radians + PI);
        Transform { translation: Vec3::new(new.x, new.y, z), rotation, ..default() }
    }
    pub fn index_to_transform(&self, index: usize, origin_offset: f32, z: f32, tile_width: usize) -> Transform {
        assert!(index < self.tile_places(), "Index needs to be less than the amount of tile places on the planet");
        
        let a = if tile_width % 2 == 0 { self.angular_step() / 2.0 } else { 0.0 };
        let radians = index as f32 * self.angular_step() + a;
        self.radians_to_transform(radians, origin_offset, z)
    }
    pub fn normalize_radians(angle: f32) -> f32 {
        ((angle % TAU) + TAU) % TAU
    }
    pub fn radians_to_index(&self, radians: f32) -> usize {
        let radians = Self::normalize_radians(radians);
        ((radians / self.angular_step()) as usize).min(self.tile_places() - 1)
    }
    /// Returns a vector of position indices that are within a certain
    /// radius of a position index. E.g if we have radius = 2, and the
    /// position index is 5, we will get [3, 4, 5, 6, 7]. It also wraps
    /// around the `planet.tile_places()` amount of tiles. So if we have
    /// radius = 2, and the position index is 0, we will get [98, 99, 0, 1, 2].
    pub fn numbers_in_radius(&self, position_index: usize, radius: usize) -> Vec<usize> {
        let mut indices = Vec::with_capacity(radius * 2 + 1);
        
        for i in (position_index as isize - radius as isize)..=(position_index as isize + radius as isize) {
            let mut index = i;
            if index < 0 {
                index += self.tile_places() as isize;
            } else if index >= self.tile_places() as isize {
                index -= self.tile_places() as isize;
            }
            indices.push(index as usize);
        }

        indices
    }
    pub fn number_is_in_radius(&self, position_index: usize, radius: usize, number: usize) -> bool {
        let clockwise_distance = (position_index + self.tile_places() - number) % self.tile_places();
        let counterclockwise_distance = (number + self.tile_places() - position_index) % self.tile_places();
        clockwise_distance <= radius || counterclockwise_distance <= radius
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

    // | ^^ ------- RADIANS & RADIUS RELATED ------- ^^ | \\
    // | ^^ ------- RADIANS & RADIUS RELATED ------- ^^ | \\

    /// Generates planet POI:s
    fn generate_pois(&mut self, commands: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> () {
        PointOfInterest::spawn_multiple(PointOfInterestType::Stone(Stone))
            .with_origin_offset(-15.0)
            .with_z_index(-1.5)
            .with_probability(0.3)
            .with_local_seed(1)
            .spawn_all(commands, asset_server, self);
        
        PointOfInterest::spawn_multiple(PointOfInterestType::Tree(Tree::new()))
            .with_origin_offset(-1.0)
            .with_z_index(-2.0)
            .with_probability(0.4)
            .with_local_seed(0)
            .spawn_all(commands, asset_server, self);
    }

    /// If two tiles are connected via cables
    pub fn powergrid_tiles_are_connected(&self, a: usize, b: usize) -> bool {
        match self.tiles.get(&a) {
            Some(e) => e.powergrid_status().connected_tiles.contains(&b),
            None => false,
        }
    }
    pub fn powergrid_register_connection(&mut self, a: usize, b: usize) -> () {
        if let Some(e) = self.tiles.get_mut(&a) { e.powergrid_status_mut().connected_tiles.push(b); }
        if let Some(e) = self.tiles.get_mut(&b) { e.powergrid_status_mut().connected_tiles.push(a); }
    }
}

#[derive(Resource, Default)]
pub struct CameraPlanetRotation {
    pub radians: f32,
}
pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                Material2dPlugin::<PlanetMaterial>::default(),
                Material2dPlugin::<PlanetAtmosphereMaterial>::default(),
            ))
            .init_resource::<CameraPlanetRotation>()
            .init_resource::<PlanetConfiguration>()
            .register_type::<PlanetConfiguration>()
            .add_plugins(ResourceInspectorPlugin::<PlanetConfiguration>::default())
            .add_systems(Startup, Planet::setup)
            .add_systems(Update, (Planet::update, debug::on_update))
            .add_systems(FixedUpdate, Self::tick);
    }
}

impl PlanetPlugin {
    /// Ticks every planet
    fn tick(mut planets: Query<&mut Planet>) -> () {
        for mut planet in planets.iter_mut() {
            let mut energy_to_add: HashMap<usize, f32> = HashMap::new();
            let tile_keys = planet.tiles.keys().cloned().collect::<Vec<usize>>();
            for key in tile_keys {
                let tile = planet.tiles.get(&key).unwrap();
                if let Some(energy_output) = tile.energy_output() {
                    Tile::distribute_energy_from(
                        tile.tile_id,
                        energy_output,
                        &mut energy_to_add,
                        &mut planet
                    );
                }
            }

            // Apply the energy to tiles
            for (tile_id, energy) in energy_to_add {
                Tile::add_energy(&mut planet, tile_id, energy);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planet_initialization_radii() {
        let config = PlanetConfiguration {
            resolution: 100,
            ..default()
        };
        let radii = Planet::get_surface_radii(&config);
        assert_eq!(radii.len(), config.resolution);
    }

    #[test]
    fn planet_initialization_radii_values() {
        let config = PlanetConfiguration {
            resolution: 100,
            seed: rand::random(),
            ..default()
        };
        let radii = Planet::get_surface_radii(&config);
        for (_, radius) in radii {
            assert!(radius >= config.radius - config.amplitude);
            assert!(radius <= config.radius + config.amplitude);
        }
    }

    #[test]
    fn planet_initialization_radii_values_seed() {
        let config = PlanetConfiguration {
            resolution: 100,
            seed: 1,
            ..default()
        };
        let radii = Planet::get_surface_radii(&config);
        let radii_2 = Planet::get_surface_radii(&config);
        for (i, (angle, radius)) in radii.iter().enumerate() {
            let (angle_2, radius_2) = radii_2[i];
            assert_eq!(*angle, angle_2);
            assert_eq!(*radius, radius_2);
        }
    }

    #[test]
    fn number_in_radius() {
        let planet = Planet {
            radius: 1000.0,
            resolution: 100,
            radii: vec![(0.0, 100.0), (0.0, 100.0), (0.0, 100.0)],
            ..default()
        };

        let tp = planet.tile_places() - 1;
        assert_eq!(planet.numbers_in_radius(0, 0), vec![0]);
        assert_eq!(planet.numbers_in_radius(5, 0), vec![5]);
        assert_eq!(planet.numbers_in_radius(0, 1), vec![tp, 0, 1]);
        assert_eq!(planet.numbers_in_radius(0, 2), vec![tp - 1, tp, 0, 1, 2]);
        assert_eq!(planet.numbers_in_radius(0, 3), vec![tp - 2, tp - 1, tp, 0, 1, 2, 3]);
        assert_eq!(planet.numbers_in_radius(5, 2), vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn number_is_in_radius() {
        let planet = Planet {
            radius: 1000.0,
            resolution: 100,
            radii: vec![(0.0, 100.0), (0.0, 100.0), (0.0, 100.0)],
            ..default()
        };

        let tp = planet.tile_places() - 1;
        assert!(planet.number_is_in_radius(0, 0, 0));
        assert!(planet.number_is_in_radius(0, 1, tp));
        assert!(planet.number_is_in_radius(0, 2, tp - 1));
        assert!(planet.number_is_in_radius(0, 2, tp - 2) == false);
    }
}
