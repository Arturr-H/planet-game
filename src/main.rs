#![allow(unused_imports, dead_code)]

/* Modules */
mod utils;
mod camera;
mod components;
mod systems;
mod functional;
mod ui;

/* Imports */
use bevy::{
    prelude::*, utils::HashSet, window::{ PresentMode, WindowTheme }
};
use camera::PIXEL_PERFECT_LAYERS;
use functional::damageable;
use systems::game::{self, GameState};
use utils::color::hex;
use components::{cable::{cable, slot}, foliage, planet::{self, mesh::{generate_planet_mesh, update_star}}, tile};

/// In-game resolution width.
pub const RES_WIDTH: f32 = 240.0 * 2.0;
/// In-game resolution height.
pub const RES_HEIGHT: f32 = 120.0 * 2.0;

fn main() {
    dotenv::dotenv().ok();
    
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

        /* Important plugins */
        .add_plugins((
            game::GameTickPlugin
        ))
        .add_plugins((
            /* Preferrable called first as many
                plugins depend on the planet existing */
            planet::PlanetPlugin,
            foliage::FoliagePlugin,

            slot::CableSlotPlugin,
            cable::CablePlugin,
            game::GamePlugin,
            damageable::DamageablePlugin,
            tile::spawn::TilePlugin,

            camera::CameraPlugin,
            camera::CameraDebugPlugin,

            // ui::hud::HudPlugin,
            
            // ui::inventory::InventoryPlugin,
        ))
        // ahhahahahahahah
        // .add_systems(Startup, generate_planet_mesh)
        // .add_systems(Update, update_star)
        .run();
}
