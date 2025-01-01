/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::slot::SLOT_SIZE, systems::{game::GameState, traits::{EnergyStorage, Generator, GenericTile, PowergridStatus}}, utils::color::hex};
use super::{debug::DebugTile, empty::EmptyTile, power_pole::PowerPole, solar_panel::SolarPanel};

/// Something that can be placed in a slot
#[enum_delegate::implement(GenericTile)]
#[derive(Component, Clone)]
pub enum Tile {
    Empty(EmptyTile),
    SolarPanel(SolarPanel),
    DebugTile(DebugTile),
    PowerPole(PowerPole)
}

impl Tile {
    pub fn random(game_state: &mut ResMut<GameState>, slot_id: usize) -> Self {
        let tile = match rand::random::<f32>() {
            x if x < 0.1 => Self::Empty(EmptyTile::new(slot_id)),
            x if x < 0.2 => Self::DebugTile(DebugTile::new(slot_id)),
            x if x < 0.4 => Self::SolarPanel(SolarPanel::new(slot_id)),
            _ => Self::PowerPole(PowerPole::new(slot_id))
        };

        game_state.slots.insert(slot_id, tile.clone());
        tile
    }

    /// Distributes energy from this tile if it is implementing
    /// `Generator` to tiles implementing `EnergyStorage`
    /// 
    /// Returns the ids of the slots that received energy for
    /// animation purposes
    pub fn distribute_energy(&self, game_state: &mut GameState) -> () {
        use Tile::*;

        match &self {
            SolarPanel(e) => e.distribute_energy(game_state),
            DebugTile(_) | Empty(_) => (),
            PowerPole(_) => (),
        }
    }

    /// Adds energy to all tiles implementing `EnergyStorage`
    pub fn add_energy(&mut self, energy: f32) -> () {
        use Tile::*;

        match self {
            DebugTile(e) => e.add_energy(energy),
            SolarPanel(_) | Empty(_) => (),
            PowerPole(_) => (),
        }
    }
    pub fn can_store_energy(&self) -> bool {
        use Tile::*;

        match self {
            DebugTile(_) => true,
            SolarPanel(_) | PowerPole(_) | Empty(_) => false,
        }
    }
}
