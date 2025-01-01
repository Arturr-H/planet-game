/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, systems::{game::GameState, traits::{EnergyStorage, Generator, GenericTile, PowergridStatus}}, utils::color::hex};
use super::Tile;

#[derive(Component, Clone)]
pub struct EmptyTile {
    pub slot_id: usize,
    pub powergrid_status: PowergridStatus,
}

impl GenericTile for EmptyTile {
    fn slot_id(&self) -> usize { self.slot_id }
    fn spawn(&self, commands: &mut ChildBuilder, transform: Transform, _: &Res<AssetServer>, _: &mut ResMut<GameState>) -> () {
        commands.spawn((
            transform,
            Sprite {
                color: hex!("#ffffff00"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Tile::Empty(self.clone()),
            PIXEL_PERFECT_LAYERS,
        ));
    }

    fn powergrid_status(&self) ->  &PowergridStatus { &self.powergrid_status }
    fn powergrid_status_mut(&mut self) -> &mut PowergridStatus { &mut self.powergrid_status }
}

impl EmptyTile {
    pub fn new(slot_id: usize) -> Self {
        Self {
            slot_id,
            powergrid_status: PowergridStatus::default(),
        }
    }
}
