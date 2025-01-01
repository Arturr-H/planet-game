/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, systems::{game::GameState, traits::{EnergyStorage, Generator, GenericTile, PowergridStatus}}, utils::color::hex};
use super::Tile;

#[derive(Component, Clone)]
pub struct DebugTile {
    pub slot_id: usize,
    pub energy: f32,
    pub powergrid_status: PowergridStatus,
}

impl GenericTile for DebugTile {
    fn slot_id(&self) -> usize { self.slot_id }
    fn spawn(&self, commands: &mut ChildBuilder, transform: Transform, _: &Res<AssetServer>, _: &mut ResMut<GameState>) -> () {
        commands.spawn((
            transform,
            Sprite {
                color: hex!("#ff0000"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Tile::DebugTile(self.clone()),
            PIXEL_PERFECT_LAYERS,
        ));
    }

    fn powergrid_status(&self) ->  &PowergridStatus { &self.powergrid_status }
    fn powergrid_status_mut(&mut self) -> &mut PowergridStatus { &mut self.powergrid_status }
}

impl Generator for DebugTile {
    fn output(&self) -> f32 { 1.0 }
}

impl EnergyStorage for DebugTile {
    fn add_energy(&mut self, amount: f32) {
        println!("Adding energy to DebugTile: {}", amount);
        self.energy = (self.energy + amount).min(self.capacity());
    }
    fn stored(&self) -> f32 { self.energy }
}

impl DebugTile {
    pub fn new(slot_id: usize) -> Self {
        Self {
            slot_id,
            energy: 0.0,
            powergrid_status: PowergridStatus::default(),
        }
    }
}
