/* Imports */
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use crate::{camera::OuterCamera, systems::game::GameState, RES_WIDTH};
use super::{Planet, PlanetAtmosphereMaterial, PlanetMaterial, PlayerPlanet};

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlanetConfiguration {
    pub seed: u32,
    pub radius: f32,

    #[inspector(min = 1)]
    pub resolution: usize,
    pub amplitude: f32,
    pub frequency: f64,
    pub octaves: u32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub warp_factor: f32,
}

impl Default for PlanetConfiguration {
    fn default() -> Self {
        Self {
            seed: 11,
            radius: RES_WIDTH * 0.625,
            resolution: 100,
            amplitude: 300.0,
            frequency: 100.0,
            octaves: 1,
            persistence: 14.9,
            lacunarity: 9.5,
            warp_factor: 5.0,
        }
    }
}

/// On update configuration (system)
pub fn on_update(
    config: ResMut<PlanetConfiguration>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    game_state: ResMut<GameState>,
    planet_materials: ResMut<Assets<PlanetMaterial>>,
    planet_atmosphere_materials: ResMut<Assets<PlanetAtmosphereMaterial>>,
    planet_q: Query<(&Planet, Entity), With<PlayerPlanet>>,
    camera_q: Query<(&mut Transform, Entity), With<OuterCamera>>,
    asset_server: Res<AssetServer>,
) -> () {
    if config.is_changed() {
        if let Ok((_planet, entity)) = planet_q.get_single() {
            commands.entity(entity).despawn_recursive();
            Planet::setup(
                commands,
                meshes,
                game_state,
                planet_materials,
                planet_atmosphere_materials,
                camera_q,
                config,
                asset_server
            );
        }
    }
}
