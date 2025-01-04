/* Imports */
use std::{f32::consts::{PI, TAU}, fmt::Debug};
use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::{cable::cable::Cable, debug::debug::DebugComponent, foliage::{animation::WindSwayPlugin, foliage::Foliage, tree::Tree}, tile::{Tile, TILE_SIZE}}, functional::damageable::Damageable, systems::game::{GameState, PlanetResources}, RES_HEIGHT, RES_WIDTH};

/* Constants */
pub const PLANET_RADIUS: f32 = RES_WIDTH * 0.625;
pub const PLANET_CIRCUMFERENCE: f32 = 2.0 * PI * PLANET_RADIUS;
pub const PLANET_DIAMETER: f32 = PLANET_RADIUS * 2.0;

/// The angular step between two tiles on the planet. Each tile
/// is placed somewhere on the circumference of the planet, and
/// the position of the tile is just stored as an angle. This constant
/// is the angular distance between two tiles.
pub const PLANET_ANGULAR_STEP: f32 = TILE_SIZE / PLANET_RADIUS;
pub const PLANET_TILE_PLACES: usize = (TAU / PLANET_ANGULAR_STEP) as usize;
const PLANET_ROTATION_SPEED: f32 = 1.7;
const FOLIAGE_SPAWNING_CHANCE: f32 = 0.8;

#[derive(Component)]
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
        mut game_state: ResMut<GameState>,
        asset_server: Res<AssetServer>
    ) -> () {
        let mut planet = commands.spawn((
            Sprite {
                image: asset_server.load("../assets/planet/planet.png"),
                custom_size: Some(Vec2::new(PLANET_DIAMETER, PLANET_DIAMETER)),
                ..default()
            },
            Transform::from_xyz(0.0, -(PLANET_DIAMETER / 2.0) * 1.1, 1.0)
            .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            PIXEL_PERFECT_LAYERS,
            PickingBehavior::IGNORE,
        ));

        /* Insert planet component */
        planet.insert(Planet {
            id: game_state.new_planet_id(),
            tiles: HashMap::new(),
            tile_id: 0,
            resources: PlanetResources::default(),
            planet_entity: Some(planet.id()),
        });

        // TODO: Only insert if it's the players own
        planet.insert(PlayerPlanet);

        /* Debug for knowing where 0degrees is on the planet */
        planet.with_children(|parent| {
            DebugComponent::setup(parent, "0deg", Self::degree_to_transform(0.0, 5.0, 5.0));
        });

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

    pub fn tick(&mut self) -> () {
        let keys = self.tiles.keys().cloned().collect::<Vec<usize>>();
        for key in keys {
            let tile = self.tiles.get(&key).unwrap();
            if tile.can_distribute_energy() {
                Tile::distribute_energy(
                    tile.energy_output(),
                    tile.tile_id,
                    self
                );
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

    /// Get planet entity or panic
    pub fn planet_entity(&self) -> Entity { self.planet_entity.unwrap() }

    // Helper
    pub fn degree_to_transform(degree: f32, origin_offset: f32, z: f32) -> Transform {
        let x = degree.cos() * (PLANET_DIAMETER / 2.0 + origin_offset);
        let y = degree.sin() * (PLANET_DIAMETER / 2.0 + origin_offset);
        let rotation = Quat::from_rotation_z(degree - std::f32::consts::PI / 2.0);
        Transform { translation: Vec3::new(x, y, z), rotation, ..default() }
    }

    // Get forward
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
