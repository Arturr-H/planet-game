/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, systems::{game::GameState, traits::{Generator, GenericTile, PowergridStatus}}, utils::color::hex};
use super::Tile;

#[derive(Component, Clone)]
pub struct SolarPanel {
    pub slot_id: usize,
    pub powergrid_status: PowergridStatus,
}

impl GenericTile for SolarPanel {
    fn slot_id(&self) -> usize { self.slot_id }
    fn spawn(&self, commands: &mut ChildBuilder, transform: Transform, _: &Res<AssetServer>, _: &mut ResMut<GameState>) -> () {
        commands.spawn((
            transform,
            Sprite {
                color: hex!("#ffff00"),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Tile::SolarPanel(self.clone()),
            PIXEL_PERFECT_LAYERS,
        ));
    }

    fn powergrid_status(&self) ->  &PowergridStatus { &self.powergrid_status }
    fn powergrid_status_mut(&mut self) -> &mut PowergridStatus { &mut self.powergrid_status }
}
impl Generator for SolarPanel {
    fn output(&self) -> f32 { 1.0 }
}

impl SolarPanel {
    pub fn new(slot_id: usize) -> Self {
        Self {
            slot_id,
            powergrid_status: PowergridStatus::default(),
        }
    }
}
