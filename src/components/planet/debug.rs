/* Imports */
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use crate::{camera::{CameraSettings, OuterCamera}, systems::game::GameState, RES_WIDTH};
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
}

impl Default for PlanetConfiguration {
    fn default() -> Self {
        Self {
            seed: 11,
            radius: 1400.0,
            resolution: 500,
            amplitude: 2000.0,
            frequency: 80.0,
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
    camera_q: Query<&mut Transform, With<OuterCamera>>,
    asset_server: Res<AssetServer>,
    camera_settings: Res<CameraSettings>,
) -> () {
    if config.is_changed() {
        if let Ok((_planet, entity)) = planet_q.get_single() {
            match commands.get_entity(entity) {
                Some(e) => e.despawn_recursive(),
                None => ()
            };
            Planet::setup(
                commands,
                meshes,
                game_state,
                planet_materials,
                planet_atmosphere_materials,
                camera_q,
                config,
                asset_server,
                camera_settings,
            );
        }
    }
}
