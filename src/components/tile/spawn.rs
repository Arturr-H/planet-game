/* Imports */
use std::f32::consts::PI;
use bevy::{ecs::entity, prelude::*, render::texture, utils::hashbrown::HashSet};
use crate::{camera::OuterCamera, components::{planet::{Planet, PlayerPlanet}, poi::{PointOfInterest, PointOfInterestHighlight, PointOfInterestType}}, systems::traits::GenericTile, ui::{info_text::SpawnInfoText, stats::{OpenStats, StatsPlugin}}, utils::{color::hex, logger}};
use super::{material::TileMaterialOutline, types::{battery::Battery, debug::DebugTile, drill::Drill, power_pole::PowerPole, solar_panel::SolarPanel, wind_turbine::WindTurbine}, Tile, TileType};

/* Constants */
const TILE_PREVIEW_ELEVATION: f32 = 10.0;

/// An event that is triggered when a tile should be spawned
/// (won't always suceed because of e.g not enough resources
/// or the tile is not allowed to be placed there). Will be
/// sent from system methods in struct `tile::Tile`
#[derive(Event, Clone)]
pub struct TileSpawnEvent {
    // pub tile_type: TileType,
    pub tile: Tile,

    /// If this is just a preview tile (will be despawned)
    pub is_preview: bool,

    /// If this tile spawn event is a tile upgrade - if so,
    /// we won't insert the new tile into the planet tiles.
    /// We'll just increase the tile_level of the existing tile.
    pub upgrade: bool,

    pub play_sound: bool,
}

/// Some bevy system parameters that are passed to the
/// `spawn` method in the `GenericTile` trait
pub struct TileSpawnEventParams<'a> {
    pub asset_server: Res<'a, AssetServer>,
    pub texture_atlas_layouts: ResMut<'a, Assets<TextureAtlasLayout>>,
    pub planet: Mut<'a, Planet>,
    pub meshes: ResMut<'a, Assets<Mesh>>,
    pub outline_material : ResMut<'a, Assets<TileMaterialOutline>>,
}

/// A component that is added to the preview tile (marker)
#[derive(Component)]
pub struct TilePreview {
    tile_type: TileType
}

pub struct TileSpawnPlugin;
impl TileSpawnPlugin {
    fn event_listener(
        mut tile_spawn_events: EventReader<TileSpawnEvent>,
        mut commands: Commands,
        mut planet_q: Query<&mut Planet, With<PlayerPlanet>>,
        asset_server: Res<AssetServer>,
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        preview_q: Query<Entity, With<TilePreview>>,
        meshes: ResMut<Assets<Mesh>>,
        outline_material : ResMut<Assets<TileMaterialOutline>>,
    ) {
        let Ok(planet) = planet_q.get_single_mut() else { return };
        let planet_entity = planet.planet_entity();
        let mut spawn_params = TileSpawnEventParams {
            asset_server,
            texture_atlas_layouts,
            planet,
            meshes,
            outline_material
        };

        for spawn_data in tile_spawn_events.read() {
            println!("3");
            let mut tile_entity = None;

            // [PREVIEW] Spawn tile
            if spawn_data.is_preview {
                commands.entity(planet_entity).with_children(|parent| {
                    tile_entity = Some(spawn_data.tile.tile_type.spawn(parent, &mut spawn_params, &spawn_data));
                });

                if let Some(entity) = tile_entity {
                    logger::log::bright_green("tile_spawn", &format!("Spawned preview of {:?}", spawn_data.tile.tile_type.display_name()));
                    commands.entity(entity).insert(TilePreview {
                        tile_type: spawn_data.tile.tile_type.clone()
                    });
                }
            }

            // Spawn tile
            else {
                println!("4");
                if !spawn_data.upgrade {
                    // Check if position is occupied
                    let false = &spawn_params.planet.tiles.values()
                        .any(|tile| tile.tile_id == spawn_data.tile.tile_id) 
                    else {
                        logger::log::red("tile_plugin", "Position is occupied");
                        return
                    };

                    // If we have enough distance from other tiles
                    // that require it
                    if !Self::is_keeping_distance_from(
                        &spawn_params.planet,
                        &spawn_data.tile.tile_type.keep_distance_from(),
                        spawn_data.tile.tile_id
                    ).is_empty() {
                        logger::log::red("tile_plugin", "Not enough distance from other tiles");
                        return
                    };
                    
                    // If the tile fits within the tile grid
                    if Self::tile_fits(
                        &spawn_params.planet,
                        &spawn_data.tile.tile_type.width(),
                        spawn_data.tile.tile_id
                    ) == false {
                        logger::log::red("tile_plugin", "Tile does not fit");
                        return
                    };

                    // If we have enough resources - spend them
                    if let Err(e) = spawn_params.planet.resources.try_spend(&spawn_data.tile.tile_type.cost()) {
                        logger::log::red("tile_plugin", e);
                        return
                    };
                }
                println!("5");

                // Play sound
                if spawn_data.play_sound {
                    let place_sound = spawn_params.asset_server.load("../assets/audio/place.wav");
                    commands.spawn((AudioPlayer::new(place_sound), PlaybackSettings::DESPAWN));
                }

                println!("6");

                // If we're upgrading a tile, the tile has already been
                // removed by the `TileUpgradeCommand` command
                commands.entity(planet_entity).with_children(|parent| {
                    let mut new_spawn_data = spawn_data.clone();
                    if spawn_data.upgrade {
                        let tile = &mut spawn_params.planet.tiles.get_mut(&spawn_data.tile.tile_id).unwrap();
                        tile.tile_level += 1;
                        new_spawn_data.tile.tile_level = tile.tile_level;
                    }
                    
                    println!("7");
                    tile_entity = Some(spawn_data.tile.tile_type.spawn(parent, &mut spawn_params, &new_spawn_data));
                });

                println!("8");
                println!("{:?}", tile_entity);

                // Add / update the new tile to game state
                if spawn_data.upgrade {
                    let tile = &mut spawn_params.planet.tiles.get_mut(&spawn_data.tile.tile_id).unwrap();
                    tile.entity = tile_entity.unwrap();
                }else {
                    println!("9");
                    spawn_params.planet.tiles.insert(spawn_data.tile.tile_id, Tile::new(
                        spawn_data.tile.tile_id,
                        spawn_data.tile.tile_type.clone(),
                        spawn_data.tile.tile_level,
                        tile_entity.unwrap()
                    ));
                    println!("10");
                }

                println!("11");
                // On click method
                let tile_id = spawn_data.tile.tile_id.clone();
                commands.entity(tile_entity.unwrap()).observe(
                    move |_: Trigger<Pointer<Down>>, events: EventWriter<OpenStats>| {
                    
                    Self::on_click(tile_id, events);
                });

                println!("12");
                
                for entity in preview_q.iter() { commands.entity(entity).despawn_recursive(); }
                println!("13");
            }
        }
    }

    /// Gets the cursor position and updates the transform
    /// of the preview tile. (rotating around world matching
    /// cursor pos)
    fn update_preview(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Transform, &TilePreview), With<TilePreview>>,
        mut event_writer: EventWriter<TileSpawnEvent>,
        mb: Res<ButtonInput<MouseButton>>,
        planet_q: Query<(&Planet, &Transform), (With<Planet>, With<PlayerPlanet>, Without<TilePreview>)>,
        windows_q: Query<&Window>,
        camera_q: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    ) -> () {
        // If we have a preview active or not
        let Ok((tile_preview_entity, mut transform, TilePreview { tile_type })) = query.get_single_mut() else { return };

        let window = windows_q.single();
        let (camera, camera_transform) = camera_q.single();
        let (planet, planet_transform) = planet_q.single();
        let planet_rotation_z = planet_transform.rotation.to_euler(EulerRot::XYZ).2 - PI / 2.0;
        let planet_pos = planet_transform.translation.truncate();

        commands.queue(SpawnInfoText("Press ESC to cancel".to_string()));
        
        let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
            else { return };

        let cursor_angle = (cursor_pos - planet_pos).angle_to(Vec2::Y);
        let index = planet.radians_to_index(- planet_rotation_z - cursor_angle);
        let p = planet.index_to_transform(index, TILE_PREVIEW_ELEVATION, 2.0, tile_type.width());

        if mb.just_pressed(MouseButton::Left) {
            event_writer.send(TileSpawnEvent {
                tile: Tile::new(index, tile_type.clone(), 0, Entity::PLACEHOLDER),
                upgrade: false,
                is_preview: false,
                play_sound: true,
            });
        }

        // Highlight some POI:s (drill highlights stones etc)
        if !tile_type.interacts_with().is_empty() {
            for poi_pos_index in planet.numbers_in_radius(index, 2) {
                let Some(pois) = planet.points_of_interest.get(&poi_pos_index) else { continue };
                'inner: for poi in pois {
                    if !tile_type.interacts_with().contains(&poi.poi_type) { continue };
                    let Some(mut entity) = commands.get_entity(poi.entity) else { continue 'inner };
                    entity.insert(PointOfInterestHighlight::new());
                }
            };
        }

        // Highlight tiles that are in the way
        let keep_distance_from = tile_type.keep_distance_from();
        for entity in Self::is_keeping_distance_from(planet, &keep_distance_from, index) {
            if let Some(mut entity) = commands.get_entity(entity) { 
                entity.insert(PointOfInterestHighlight::red());
            };
        };

        // If the tile does not fit, highlight the tile as red
        if !Self::tile_fits(&planet, &tile_type.width(), index) {
            commands.get_entity(tile_preview_entity).map(|mut e| {
                e.try_insert(PointOfInterestHighlight::red());
            });
        };

        // Offset the placement with 1 unit to make sure the object is wedged into the ground
        transform.translation = p.translation.with_z(-0.4); 
        transform.rotation = p.rotation;
    }

    /// Handles keyboard input for now (later we'll add an UI)
    fn spawn_preview(
        mut commands: Commands,
        mut event_writer: EventWriter<TileSpawnEvent>,
        preview_q: Query<Entity, With<TilePreview>>,
        kb: Res<ButtonInput<KeyCode>>,
    ) -> () {
        let mut tile: Option<TileType> = None;

        // TODO: This is just debug code, we'll add an UI for this later
        if kb.just_pressed(KeyCode::KeyQ) { tile = Some(TileType::PowerPole(PowerPole)); }
        if kb.just_pressed(KeyCode::KeyW) { tile = Some(TileType::DebugTile(DebugTile)); }
        if kb.just_pressed(KeyCode::KeyE) { tile = Some(TileType::SolarPanel(SolarPanel)); }
        if kb.just_pressed(KeyCode::KeyR) { tile = Some(TileType::Drill(Drill)); }
        if kb.just_pressed(KeyCode::KeyT) { tile = Some(TileType::Battery(Battery)); }
        if kb.just_pressed(KeyCode::KeyY) { tile = Some(TileType::WindTurbine(WindTurbine)); }
        if kb.just_pressed(KeyCode::Escape) {
            for entity in preview_q.iter() { commands.entity(entity).despawn_recursive(); }
        }

        if let Some(tile) = tile {
            // Remove previews
            for entity in preview_q.iter() { commands.entity(entity).despawn_recursive(); }

            // Spawn preview 
            event_writer.send(TileSpawnEvent {
                tile: Tile::new(0, tile.clone(), 0, Entity::PLACEHOLDER),
                upgrade: false,
                is_preview: true,
                play_sound: false,
            });
        }
    }

    /// On click method for tiles (open stats menu etc)
    fn on_click(
        tile_id: usize,
        mut events: EventWriter<OpenStats>,
    ) -> () {
        events.send(OpenStats{
            open: true, tile_id: Some(tile_id)
        });
    }

    /// If this function returns an empty vector, the tile can be placed
    /// at the given index. Otherwise, the vector contains the entities
    /// of the tiles that are in the way.
    fn is_keeping_distance_from(planet: &Planet, keep_distance_from: &Vec<(usize, TileType)>, index: usize) -> Vec<Entity> {
        let mut entities = Vec::new();
        let max_radius = keep_distance_from.iter().map(|(r, _)| r).max().unwrap_or(&0);
        for tile_pos_index in planet.numbers_in_radius(index, *max_radius) {
            let Some(tile) = planet.tiles.get(&tile_pos_index) else { continue };
            let Some((radius, _)) = keep_distance_from.iter().find(|(_, t)| *t == tile.tile_type) else { continue };

            if planet.number_is_in_radius(index, *radius, tile.tile_id) {
                entities.push(tile.entity);
            }
        };

        entities
    }

    /// Every tile_type has a width, which is the amount of tiles
    /// it occupies in the grid. This function returns an empty
    /// vector if the tile fits, otherwise it returns the entities
    /// of the tiles that are in the way.
    fn tile_fits(planet: &Planet, width: &usize, index: usize) -> bool {
        let mut occupied = HashSet::new();

        for i in planet.numbers_in_radius(index, 5) {
            if let Some(tile) = &planet.tiles.get(&i) {
                for position in Tile::get_tile_spread(tile.tile_type.width(), i, planet.tile_places()) {
                    occupied.insert(position);
                }
            }
        }

        for position in Tile::get_tile_spread(*width, index, planet.tile_places()) {
            if occupied.contains(&position) {
                return false
            }
        }
    
        true
    }
}

pub struct SpawnTileCommand {
    pub tile_type: TileType,
    pub tile_id: usize,
    pub play_sound: bool,
}

impl Command for SpawnTileCommand {
    fn apply(self, world: &mut World) {
        world.send_event(TileSpawnEvent {
            tile: Tile::new(self.tile_id, self.tile_type, 0, Entity::PLACEHOLDER),
            upgrade: false,
            is_preview: false,
            play_sound: self.play_sound,
        });
    }
}

impl Plugin for TileSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::event_listener, Self::spawn_preview,
                Self::update_preview
            ))
            .add_event::<TileSpawnEvent>();
    }
}
