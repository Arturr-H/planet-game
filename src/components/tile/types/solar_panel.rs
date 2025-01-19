/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::{cable::slot::CableSlot, tile::{spawn::{TileSpawnEvent, TileSpawnEventParams}, Tile}}, systems::{game::PlanetResource, traits::GenericTile}};

/// A solar panel is a tile that generates energy
/// if sun is shining on it.
#[derive(Component, Clone, Debug)]
pub struct SolarPanel;
impl GenericTile for SolarPanel {
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        spawn_params: &mut TileSpawnEventParams,
        spawn_data: &TileSpawnEvent,
    ) -> Entity {
        let transform = spawn_params.planet.index_to_transform(
            spawn_data.tile.tile_id, 0.0, 1.0, spawn_data.tile.tile_type.width());
        
        if !spawn_data.is_preview {
            CableSlot::spawn(
                commands, &spawn_params.asset_server, spawn_data.tile.tile_id, transform
            );
        }

        commands.spawn((
            transform,
            Sprite {
                image: spawn_params.asset_server.load(format!("machines/solar_panel/0{}.png", spawn_data.tile.tile_level)),
                anchor: Anchor::BottomCenter,
                // custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            SolarPanel,
        )).id()
    }

    fn display_name(&self) -> String { "Solar panel".to_string() }
    fn energy_output(&self, tile: &Tile) -> Option<f32> {
        Some(match tile.tile_level {
            0 => 1.0,
            1 => 2.0,
            2 => 3.0,
            3 => 4.0,
            4 => 5.0,
            _ => unimplemented!()
        })
    }
    fn upgrades(&self) -> Vec<Vec<(PlanetResource,usize)> > {
        vec![
            vec![(PlanetResource::Wood, 4)],
            vec![(PlanetResource::Stone, 4)],
            vec![(PlanetResource::Stone, 4)],
            vec![(PlanetResource::Stone, 4)],
            vec![(PlanetResource::Stone, 4)],
        ]
    }

    fn cost(&self) -> Vec<(PlanetResource, usize)> {
        vec![
            (PlanetResource::Wood, 4)
        ]
    }
}
