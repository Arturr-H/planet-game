/* Imports */
use super::state::SlotCablePlacementResource;
use crate::{
    camera::{InGameCamera, OuterCamera, HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS}, components::{cable::cable::{Cable, CablePreview, MAX_CABLE_LENGTH}, planet::planet::Planet, tile::Tile}, systems::traits::GenericTile, utils::{color::hex, sprite_bounds::point_in_sprite_bounds}, GameState, RES_HEIGHT, RES_WIDTH
};
use bevy::{prelude::*, text::FontSmoothing};

/* Constants */
pub const SLOT_SIZE: f32 = 20.0;
const OUTLINE_SIZE: f32 = 6.0;

const SLOT_ACTIVE_COLOR: &'static str = "#00000066";
const SLOT_INACTIVE_COLOR: &'static str = "#00000000";
const SLOT_HIGHLIGHT_COLOR: &'static str = "#00000044";

pub struct SlotPlugin;
impl Plugin for SlotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Slot::breathe)
        .init_resource::<SlotCablePlacementResource>();
    }
}

// Marker component for our square
#[derive(Component)]
pub struct Slot { id: usize }

impl Slot {
    /// [UTILITY] Spawns a slot
    pub fn spawn(
        tile: Tile,
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        game_state: &mut ResMut<GameState>,
        id: usize,
        transform: Transform,
    ) -> () {
        commands.spawn((
            transform.with_translation(transform.translation.with_z(0.1)),
            Slot { id },
            Sprite {
                color: hex!(SLOT_INACTIVE_COLOR),
                custom_size: Some(Vec2::new(SLOT_SIZE, SLOT_SIZE)),
                ..default()
            },
        ))
        
        /* Slot selection outline 
            ! WARNING: Needs to be the first child */
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: Color::NONE,
                    custom_size: Some(Vec2::new(
                        SLOT_SIZE + OUTLINE_SIZE,
                        SLOT_SIZE + OUTLINE_SIZE,
                    )),
                    image: asset_server.load("../assets/planet/selection.png"),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 2.0),
                HIGH_RES_LAYERS,
            ));
        })
        /* Important to set propagate to false on
            all of these observers because sometimes
            we have slots nested in slots => double events */
        .observe(Self::on_pointer_over)
        .observe(Self::on_pointer_out)
        .observe(Self::on_click);
        
        /* Tile */
        tile.spawn(commands, transform, asset_server, game_state);
    }

    /// On click
    fn on_click(
        mut click: Trigger<Pointer<Click>>,
        mut commands: Commands,
        planet_q: Query<Entity, With<Planet>>,
        mut slots_q: Query<(&Slot, &mut Sprite, &Children, &Transform), With<Slot>>,
        mut children_q: Query<&mut Sprite, Without<Slot>>,
        mut slot_res: ResMut<SlotCablePlacementResource>,
        mut game_state: ResMut<GameState>,
        cable_preview_q: Query<Entity, With<CablePreview>>
    ) {
        click.propagate(false);

        let planet = planet_q.single();
        let mut highlight_all = false;
        let mut needs_highlight_reset = false;
        
        if let Ok((slot, mut sprite, children, transform)) = slots_q.get_mut(click.entity()) {
            if let Some((id, other_entity)) = slot_res.active() {
                Cable::remove_previews(&mut commands, cable_preview_q);
                needs_highlight_reset = true;
                let occupied = game_state.powergrid_tiles_are_connected(id, slot.id);

                if occupied
                    || id == slot.id
                    || slot_res.start_entity_pos.distance(transform.translation.truncate().xy()) > MAX_CABLE_LENGTH {
                    slot_res.reset();
                }else {
                    /* Spawn cable */
                    commands.entity(planet).with_children(|parent| {
                        Cable::spawn_between_slots(parent, click.entity(), other_entity);
                    });

                    /* Register connection to game state and reset */
                    game_state.powergrid_register_connection(id, slot.id);
                    slot_res.reset();
                }
            } else {
                slot_res.set_active(slot.id, click.entity(), transform.translation);
                Self::highlight(
                    Some(true),
                    Some(true),
                    &mut sprite,
                    children,
                    &mut children_q,
                    &slot_res
                );
                Cable::spawn_preview(&mut commands, click.entity());

                highlight_all = true;
            }
        }

        /* Clear all highlights */
        if needs_highlight_reset {
            Self::clear_all_highlight(&mut slots_q, &mut children_q, &slot_res);
        }

        if highlight_all {
            Self::highlight_all(true, slots_q);
        }else {
            Self::highlight_all(false, slots_q);
        }
    }

    // System to handle hover detection and highlighting
    fn on_pointer_over(
        mut hover: Trigger<Pointer<Over>>,
        mut slots_q: Query<(&Slot, &mut Sprite, &Children, &Transform), With<Slot>>,
        mut children_q: Query<&mut Sprite, Without<Slot>>,
        slot_res: ResMut<SlotCablePlacementResource>,
    ) {
        hover.propagate(false);
        if let Ok((_, mut sprite, children, _)) = slots_q.get_mut(hover.entity()) {
            Self::highlight(
                Some(true),
                None,
                &mut sprite,
                &children,
                &mut children_q,
                &slot_res
            );
        }
    }
    fn on_pointer_out(
        mut hover: Trigger<Pointer<Out>>,
        mut slots_q: Query<(&Slot, &mut Sprite, &Children, &Transform), With<Slot>>,
        mut children_q: Query<&mut Sprite, Without<Slot>>,
        slot_res: ResMut<SlotCablePlacementResource>,
    ) {
        hover.propagate(false);
        if let Ok((_, mut sprite, children, _)) = slots_q.get_mut(hover.entity()) {
            Self::highlight(
                Some(false),
                None,
                &mut sprite,
                &children,
                &mut children_q,
                &slot_res
            );
        }
    }
    fn highlight(
        change_highlight: Option<bool>,
        change_selection: Option<bool>,
        sprite: &mut Sprite,
        children: &Children,
        children_q: &mut Query<&mut Sprite, Without<Slot>>,
        slot_res: &ResMut<SlotCablePlacementResource>
    ) -> () {
        let mut outline = children_q.get_mut(children[0]).unwrap();
        let highlight_all = slot_res.active().is_some();

        /* Inner color change */
        if let Some(highlight) = change_highlight {
            if highlight {
                sprite.color = hex!(SLOT_ACTIVE_COLOR);
            } else {
                if highlight_all {
                    sprite.color = hex!(SLOT_HIGHLIGHT_COLOR);
                }else {
                    sprite.color = hex!(SLOT_INACTIVE_COLOR);
                }
            }
        }

        /* Outer selection image */
        if let Some(selection) = change_selection {
            if selection {
                outline.color = hex!(SLOT_ACTIVE_COLOR);
            } else {
                outline.color = Color::NONE;
            }
        }
    }
    fn highlight_all(
        highlight: bool,
        mut slot_q: Query<(&Slot, &mut Sprite, &Children, &Transform), With<Slot>>,
    ) -> () {
        for (_, mut sprite, _, _) in slot_q.iter_mut() {
            if highlight {
                sprite.color = hex!(SLOT_HIGHLIGHT_COLOR);
            } else {
                sprite.color = hex!(SLOT_INACTIVE_COLOR);
            }
        }
    }
    fn clear_all_highlight(
        slots_q: &mut Query<(&Slot, &mut Sprite, &Children, &Transform), With<Slot>>,
        children_q: &mut Query<&mut Sprite, Without<Slot>>,
        slot_res: &ResMut<SlotCablePlacementResource>
    ) -> () {
        for (_, mut sprite, children, _) in slots_q.iter_mut() {
            Self::highlight(
                Some(false),
                Some(false),
                &mut sprite, &children, children_q,
                slot_res
            );
        }
    }

    // Idle animation
    fn breathe(time: Res<Time>, mut query: Query<(&mut Transform, &Slot)>) {
        for (mut transform, slot) in query.iter_mut() {
            let scale = 1.0 + (time.elapsed_secs() * 2.0 + slot.id as f32 * 12.124511).sin() * 0.1;
            transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}
