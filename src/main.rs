#![allow(dead_code, unused_imports)]

/* Modules */
mod utils;
mod camera;
mod components;
mod systems;
mod functional;
mod ui;

/* Imports */
use bevy::{
    picking, prelude::*, window::{ PresentMode, WindowTheme },
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
};
use camera::background::background::BackgroundPlugin;
use functional::damageable;
use systems::game;
use components::{cable::{cable, slot}, foliage::animation::FoliageAnimationPlugin, planet, player::player::PlayerPlugin, poi::PointOfInterestPlugin, tile};
use utils::color::hex;

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
            game::GameTickPlugin,
            camera::CameraPlugin,
        ))
        .add_plugins((
            /* Preferrable called first as many
                plugins depend on the planet existing */
            planet::PlanetPlugin,
            BackgroundPlugin,

            slot::CableSlotPlugin,
            cable::CablePlugin,
            game::GamePlugin,
            damageable::DamageablePlugin,
            FoliageAnimationPlugin,

            ui::hud::HudPlugin,
            ui::stats::StatsPlugin,
            // ui::inventory::InventoryPlugin,
            ui::info_text::InfoTextPlugin,
            PointOfInterestPlugin,
            tile::TilePlugin,
            PlayerPlugin,
        ))

        /* Debug */
        .add_plugins((
            camera::CameraDebugPlugin,

            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 16.0,
                        ..Default::default()
                    },
                    text_color: hex!("#ffffff"),
                    enabled: true,
                },
            },
        ))
        .run();
}
