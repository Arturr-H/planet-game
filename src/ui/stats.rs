use bevy::prelude::*;

use crate::{camera::UI_LAYERS, components::{planet::{Planet, PlayerPlanet}, tile::{upgrade::UpgradeTileCommand, RemoveTileCommand, Tile}}, systems::traits::GenericTile, utils::color::hex};

#[derive(Event, Resource, Clone)]
pub struct OpenStats {
    pub open: bool,
    pub tile_id: Option<usize>,
}

#[derive(Default, Resource)]
struct StatsUIState {
    stats: Option<OpenStats>,
}

#[derive(Component)]
struct StatsUI;

#[derive(Component)]
struct Label;

#[derive(Component)]
struct TileUpgradeButton;

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            // .init_resource::<Events<OpenStats>>()
            .add_event::<OpenStats>()
            .init_resource::<StatsUIState>()
            .add_systems(Update, update);
    }
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(50.0),
            height: Val::Vh(15.0),
            bottom: Val::Px(10.0),
            left: Val::Vw(25.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(hex!("#503010")),
        StatsUI,
        UI_LAYERS,
        Visibility::Hidden,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Type: [], Energy: 0"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            Label,
        ));

        /* Delete button */
        parent.spawn((
            Transform::from_xyz(0.0, 10.0, 10.0),
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
        )).with_child((
            Text::new("Delete"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ))
        .observe(on_delete);

        /* Upgrade tile button */
        parent.spawn((
            Transform::from_xyz(0.0, 10.0, 10.0),
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            TileUpgradeButton
        )).with_child((
            Text::new("Upgrade"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ))
        .observe(on_upgrade);
    });
}

fn update(
    mut events: EventReader<OpenStats>,
    mut query: Query<&mut Visibility, With<StatsUI>>,
    mut label: Query<&mut Text, With<Label>>,
    mut ui_state: ResMut<StatsUIState>,
    mut planet_q: Query<&mut Planet, With<PlayerPlanet>>,

    mut tile_upgrade_button: Query<(&mut Visibility, &Children), (With<TileUpgradeButton>, Without<Label>, Without<StatsUI>)>,
    mut tile_upgrade_button_text: Query<&mut Text, (With<TileUpgradeButton>, Without<Label>, Without<StatsUI>)>,
) {
    let planet = planet_q.single_mut();

    for event in events.read() {
        for mut visibility in query.iter_mut() {
            *visibility = if event.open {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        if event.open {
            ui_state.stats = Some(event.clone());
        } else {
            ui_state.stats = None;
        }
    }
    
    let Some(stats) = &ui_state.stats else { return };
    let Some(tile_id) = &stats.tile_id else { return };
    
    let tile = planet.tiles[tile_id].clone();
    for mut text in &mut label {
        text.0 = format!("{}\nEnergy: {}\n Level: {}",
            tile.tile_type.display_name(),
            tile.powergrid_status.energy_stored,
            tile.tile_level,
        );
    }

    for (mut visibility, children) in &mut tile_upgrade_button {
        if tile.tile_level > tile.tile_type.upgrades().len() {
            *visibility = Visibility::Hidden;
        }

        // Update the text of the upgrade button
        if let Ok(mut text) = tile_upgrade_button_text.get_mut(children[0]) {
            text.0 = format!("Upgrade (costs {})", tile.tile_type.upgrades()[tile.tile_level].iter().map(|(k, v)| {
                format!("{}x {:?}, ", v, k)
            }).collect::<String>());
        }
    }

}

fn on_delete(
    _: Trigger<Pointer<Down>>,
    mut commands: Commands,
    mut events: EventWriter<OpenStats>,
    ui_state: Res<StatsUIState>,
) -> () {
    if let Some(stats) = &ui_state.stats {
        if let Some(tile_id) = stats.tile_id {
            commands.queue(RemoveTileCommand { tile_id });
        }
    }
    events.send(OpenStats { open: false, tile_id: None });
}
fn on_upgrade(
    _: Trigger<Pointer<Down>>,
    mut commands: Commands,
    mut events: EventWriter<OpenStats>,
    ui_state: Res<StatsUIState>,
) -> () {
    if let Some(stats) = &ui_state.stats {
        if let Some(tile_id) = stats.tile_id {
            commands.queue(UpgradeTileCommand { tile_id });
        }
    }
    events.send(OpenStats { open: false, tile_id: None });
}
