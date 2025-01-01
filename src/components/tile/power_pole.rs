/* Imports */
use bevy::prelude::*;
use crate::{camera::PIXEL_PERFECT_LAYERS, components::slot::Slot, systems::{game::GameState, traits::{GenericTile, PowergridStatus}}, utils::color::hex};
use super::{empty::EmptyTile, solar_panel::SolarPanel, Tile};

/* Constants */
const POWER_SLOT_OFFSET: f32 = 42.0;

/// PowerPole component
#[derive(Component, Clone)]
pub struct PowerPole {
    pub slot_id: usize,
    pub powergrid_status: PowergridStatus,
}

impl GenericTile for PowerPole {
    fn slot_id(&self) -> usize { self.slot_id }
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        transform: Transform,
        asset_server: &Res<AssetServer>,
        game_state: &mut ResMut<GameState>
    ) -> () {
        let forward = transform.rotation * Vec3::Y;
        let forward_2d = Vec2::new(forward.x, forward.y).normalize().extend(0.0);

        /* Power pole sprite */
        commands.spawn((
            Sprite {
                image: asset_server.load("machines/power-pole.png"),
                ..default()
            },
            transform.with_translation((transform.translation + forward_2d * 15.0).with_z(-0.4)),
            Tile::PowerPole(self.clone()),
            PIXEL_PERFECT_LAYERS,
        ));

        /* We're creating a new slot thus we need to
            update the game_state to account for that */
        let slot_id = game_state.new_slot_id();
        let tile = Tile::Empty(EmptyTile::new(slot_id));
        game_state.slots.insert(slot_id, tile.clone());

        Slot::spawn(
            tile,
            commands, asset_server, game_state, slot_id,
            transform.with_translation(transform.translation + forward_2d * POWER_SLOT_OFFSET)
        );
    }

    fn powergrid_status(&self) ->  &PowergridStatus { &self.powergrid_status }
    fn powergrid_status_mut(&mut self) -> &mut PowergridStatus { &mut self.powergrid_status }
}

impl PowerPole {
    pub fn new(slot_id: usize) -> Self {
        Self {
            slot_id,
            powergrid_status: PowergridStatus::default(),
        }
    }
}
