/* Imports */
use bevy::{prelude::*, utils::HashMap};
use crate::{camera::UI_LAYERS, utils::color::hex};

use super::slot::InventorySlot;

/* Constants */
pub struct Inventory {
    // pub items: HashMap<usize, Item>,
}

impl Inventory {
    pub fn setup(
        mut commands: Commands,
        _asset_server: Res<AssetServer>,
        mut _materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(hex!("#000000aa")),
            // Visibility::Hidden,
            UI_LAYERS,
        )).with_children(|parent| {
            parent.spawn((
                Node {
                    aspect_ratio: Some(0.7),
                    height: Val::Percent(80.0),

                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),

                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(15.0)),
                BackgroundColor(hex!("#2d2d2d")),
            )).with_children(|parent| {
                /* Row */
                for col in 0..4 {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            column_gap: Val::Px(5.0),
                            ..default()
                        },
                        BackgroundColor(hex!("#ff77aa00")),
                    )).with_children(|parent| {
                        for cell in 0..3 {
                            /* Elements */
                            InventorySlot::spawn(parent, col * 3 + cell);
                        }
                    });
                }
            });
        });
    }
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Inventory::setup);
    }
}
