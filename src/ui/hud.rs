/* Imports */
use bevy::prelude::*;

use crate::{camera::UI_LAYERS, components::planet::planet::{Planet, PlayerPlanet}, systems::game::{GameState, PlanetResource}};
pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}

/* Systems */
fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(5.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        UI_LAYERS
    ))
    .insert(PickingBehavior::IGNORE)
    .with_children(|parent| {
        // left vertical fill (border)
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(5.)),
                    row_gap: Val::Px(5.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|parent| {
                // text
                parent.spawn((
                    Text::new("Wood: 0"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    Label,
                ));
            });
        });
    });
}

fn update(
    planet_q: Query<&Planet, With<PlayerPlanet>>,
    mut query: Query<&mut Text, With<Label>>,
) {
    // TODO: Something like Resouce.is_changed()
    let planet = planet_q.single();
    for mut text in &mut query {
        text.0 = format!("Wood: {}", planet.resources.get(PlanetResource::Wood));
    }
}
