#![allow(unused_imports, dead_code)]

/* Modules */
mod utils;
mod camera;
mod components;
mod systems;
mod functional;

/* Imports */
use bevy::{
    prelude::*, utils::HashSet, window::{ PresentMode, WindowTheme }
};
use camera::PIXEL_PERFECT_LAYERS;
use functional::damageable;
use systems::game::{self, GameState};
use utils::color::hex;
use components::{cable::{cable, slot}, planet::planet, tile};

/// In-game resolution width.
pub const RES_WIDTH: f32 = 240.0 * 2.0;
/// In-game resolution height.
pub const RES_HEIGHT: f32 = 120.0 * 2.0;

fn main() {
    App::new()
        /* Default */
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "game".into(),
                    name: Some("game.app".into()),
                    resolution: (RES_WIDTH * 2., RES_HEIGHT * 2.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((
            slot::CableSlotPlugin,
            cable::CablePlugin,
            planet::PlanetPlugin,
            game::GamePlugin,
            damageable::DamageablePlugin,
            tile::plugin::TilePlugin,

            camera::CameraPlugin,
            camera::CameraDebugPlugin
        ))
        .run();
}
