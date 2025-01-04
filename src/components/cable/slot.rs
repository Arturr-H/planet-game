/* Imports */
use super::slot_state::SlotCablePlacementResource;
use crate::{
    camera::{InGameCamera, OuterCamera, HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS}, components::{cable::cable::{Cable, CablePreview, MAX_CABLE_LENGTH}, planet::planet::{Planet, PlayerPlanet}, tile::{Tile, TileType, TILE_SIZE}}, utils::{color::hex, logger, sprite_bounds::point_in_sprite_bounds}, GameState
};
use bevy::{prelude::*, sprite::Anchor, text::FontSmoothing};

/* Constants */
const OUTLINE_SIZE: f32 = 6.0;
const SLOT_ACTIVE_COLOR: &'static str = "#00000066";
const SLOT_INACTIVE_COLOR: &'static str = "#00000000";
const SLOT_HIGHLIGHT_COLOR: &'static str = "#00000044";

pub struct CableSlotPlugin;
impl Plugin for CableSlotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (CableSlot::breathe, CableSlot::on_cancel))
        .init_resource::<SlotCablePlacementResource>();
    }
}

// A slot is a square where cables can be connected to
#[derive(Component, Clone)]
pub struct CableSlot {
    pub tile_id: usize,
}

impl CableSlot {
    /// [UTILITY] Spawns a slot
    pub fn spawn(
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        tile_id: usize,
        transform: Transform,
    ) -> () {
        // Slot sprite (light gray if hovered)
        commands.spawn((
            transform.with_translation(transform.translation.with_z(10.0)),
            Self { tile_id },
            Sprite {
                color: hex!(SLOT_INACTIVE_COLOR),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                anchor: Anchor::BottomCenter,
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
                        TILE_SIZE + OUTLINE_SIZE,
                        TILE_SIZE + OUTLINE_SIZE,
                    )),
                    anchor: Anchor::BottomCenter,
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
    }

    /// On click
    fn on_click(
        mut click: Trigger<Pointer<Click>>,
        mut commands: Commands,
        mut planet_q: Query<&mut Planet, With<PlayerPlanet>>,
        mut slots_q: Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
        mut children_q: Query<&mut Sprite, Without<Self>>,
        mut slot_res: ResMut<SlotCablePlacementResource>,
        cable_preview_q: Query<Entity, With<CablePreview>>
    ) {
        click.propagate(false);

        let mut planet = planet_q.single_mut();
        let mut highlight_all = false;
        let mut needs_highlight_reset = false;
        
        if let Ok((slot, mut sprite, children, transform)) = slots_q.get_mut(click.entity()) {
            if let Some((id, other_entity)) = slot_res.active() {
                Cable::remove_previews(&mut commands, cable_preview_q);
                needs_highlight_reset = true;
                let occupied = planet.powergrid_tiles_are_connected(id, slot.tile_id);

                if occupied
                    || id == slot.tile_id
                    || slot_res.start_entity_pos.distance(transform.translation.truncate().xy()) > MAX_CABLE_LENGTH {
                    slot_res.reset();
                }else {
                    logger::log::blue("cable", format!("Spawning cable between {} and {}", id, slot.tile_id));

                    /* Spawn cable */
                    commands.entity(planet.planet_entity()).with_children(|parent| {
                        Cable::spawn_between_slots(
                            parent,
                            click.entity(),
                            other_entity
                        );
                    });

                    /* Register connection to game state and reset */
                    planet.powergrid_register_connection(id, slot.tile_id);
                    slot_res.reset();
                }
            } else {
                slot_res.set_active(slot.tile_id, click.entity(), transform.translation);
                Cable::spawn_preview(&mut commands, click.entity());
                Self::highlight(
                    Some(true),
                    Some(true),
                    &mut sprite,
                    children,
                    &mut children_q,
                    &slot_res
                );

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
        mut slots_q: Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
        mut children_q: Query<&mut Sprite, Without<Self>>,
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
        mut slots_q: Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
        mut children_q: Query<&mut Sprite, Without<Self>>,
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
        children_q: &mut Query<&mut Sprite, Without<Self>>,
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
        mut slot_q: Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
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
        slots_q: &mut Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
        children_q: &mut Query<&mut Sprite, Without<Self>>,
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

    // Esc
    fn on_cancel(
        kb: Res<ButtonInput<KeyCode>>,
        mut slot_res: ResMut<SlotCablePlacementResource>,
        mut slots_q: Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
        mut children_q: Query<&mut Sprite, Without<Self>>,
        mut commands: Commands,
        cable_preview_q: Query<Entity, With<CablePreview>>
    ) {
        if kb.just_pressed(KeyCode::Escape) {
            Cable::remove_previews(&mut commands, cable_preview_q);
            Self::clear_all_highlight(&mut slots_q, &mut children_q, &slot_res);
            slot_res.reset();
        }
    }

    // Idle animation
    fn breathe(time: Res<Time>, mut query: Query<(&mut Transform, &Self)>) {
        for (mut transform, slot) in query.iter_mut() {
            let scale = 1.0 + (time.elapsed_secs() * 2.0 + slot.tile_id as f32 * 12.124511).sin() * 0.1;
            transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}
