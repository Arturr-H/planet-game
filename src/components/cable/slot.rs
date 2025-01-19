/* Imports */
use super::{cable::CableMaterial, slot_state::SlotCablePlacementResource};
use crate::{
    camera::HIGH_RES_LAYERS, components::{
        cable::cable::{Cable, CablePreview, MAX_CABLE_LENGTH},
        planet::{Planet, PlayerPlanet},
        tile::TILE_SIZE
    }, systems::game::GameState, ui::stats::OpenStats, utils::{color::hex, logger}
};
use bevy::{ecs::{entity, event}, prelude::*};

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

#[derive(Component)]
struct CableSlotColored;

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
            Self { tile_id },
            transform.with_translation(transform.translation
                .with_z(transform.translation.z + 0.1)
                + Planet::forward(&transform) * TILE_SIZE / 2.0),
            CableSlotColored,
            Sprite {
                color: hex!(SLOT_INACTIVE_COLOR),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
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
                    image: asset_server.load("../assets/planet/selection.png"),
                    ..default()
                },
                CableSlotColored,
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
        cable_preview_q: Query<Entity, With<CablePreview>>,
        mut events: EventWriter<OpenStats>,
        cable_materials: ResMut<Assets<CableMaterial>>,
        meshes: ResMut<Assets<Mesh>>,
    ) {
        click.propagate(false);

        let mut planet = planet_q.single_mut();
        let mut highlight_all = false;
        let mut needs_highlight_reset = false;
        
        if let Ok((slot, mut sprite, children, transform)) = slots_q.get_mut(click.entity()) {
            if let Some((id, other_entity)) = slot_res.active() {
                events.send(OpenStats{open: false, tile_id: None});
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
                            other_entity,
                            id,
                            slot.tile_id,
                            cable_materials,
                            meshes,
                        );
                    });

                    /* Register connection to game state and reset */
                    planet.powergrid_register_connection(id, slot.tile_id);
                    slot_res.reset();
                }
            } else {
                slot_res.set_active(slot.tile_id, click.entity(), transform.translation);
                Cable::spawn_preview(
                    &mut commands, click.entity(),
                    cable_materials, meshes,
                );
                Self::highlight(
                    Some(true),
                    Some(true),
                    &mut sprite,
                    children,
                    &mut children_q,
                    slot_res.active().is_some()
                );
                // planet.tiles[slot.tile_id]
                events.send(OpenStats{open: true, tile_id: Some(slot.tile_id)});

                highlight_all = true;
            }
        }

        /* Clear all highlights */
        if needs_highlight_reset {
            Self::clear_all_highlight(&mut slots_q, &mut children_q, slot_res.active().is_some());
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
                slot_res.active().is_some()
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
                slot_res.active().is_some()
            );
        }
    }
    fn highlight(
        change_highlight: Option<bool>,
        change_selection: Option<bool>,
        sprite: &mut Sprite,
        children: &Children,
        children_q: &mut Query<&mut Sprite, Without<Self>>,
        highlight_all: bool
    ) -> () {
        let mut outline = children_q.get_mut(children[0]).unwrap();

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
    pub fn clear_all_highlight(
        slots_q: &mut Query<(&Self, &mut Sprite, &Children, &Transform), With<Self>>,
        children_q: &mut Query<&mut Sprite, Without<Self>>,
        highlight_all: bool
    ) -> () {
        for (_, mut sprite, children, _) in slots_q.iter_mut() {
            Self::highlight(
                Some(false),
                Some(false),
                &mut sprite, &children, children_q,
                highlight_all
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
        cable_preview_q: Query<Entity, With<CablePreview>>,
        mut events: EventWriter<OpenStats>
    ) {
        if kb.just_pressed(KeyCode::Escape) {
            Cable::remove_previews(&mut commands, cable_preview_q);
            Self::clear_all_highlight(&mut slots_q, &mut children_q, slot_res.active().is_some());
            slot_res.reset();
            events.send(OpenStats{open: false, tile_id: None});
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

pub struct RemoveAllCableSlotHighlightsCommand;
impl Command for RemoveAllCableSlotHighlightsCommand {
    fn apply(self, commands: &mut World) {
        let mut slots_q = commands.query::<(&CableSlotColored, &mut Sprite)>();
        let mut cable_preview_q = commands.query_filtered::<Entity, With<CablePreview>>();
        
        /* Remove cable previews */
        commands.resource_mut::<SlotCablePlacementResource>().reset();
        if let Ok(entity) = cable_preview_q.get_single(commands) {
            DespawnRecursive { entity, warn: true }.apply(commands);
        }

        for (_, mut sprite) in slots_q.iter_mut(commands) {
            sprite.color = hex!(SLOT_INACTIVE_COLOR);
        }
    }
}

pub struct RemoveCableSlotCommand {
    pub tile_id: usize,
}
impl Command for RemoveCableSlotCommand {
    fn apply(self, commands: &mut World) {
        let tile_id = self.tile_id;

        if let Some(slot) = commands.query_filtered::<Entity, With<CableSlot>>().iter(commands).find(|&slot| {
            let slot = commands.get::<CableSlot>(slot).unwrap();
            slot.tile_id == tile_id
        }) {
            DespawnRecursive { entity: slot, warn: false }.apply(commands);
        }

        /* Remove cables previews */
        let mut cable_q = commands.query_filtered::<(Entity, &Cable), With<Cable>>();
        let mut entities_to_despawn = Vec::new();
        for (entity, cable) in cable_q.iter(commands) {
            if cable.start_tile_id == tile_id || cable.end_tile_id == tile_id {
                entities_to_despawn.push(entity);
            }
        }

        for entity in entities_to_despawn {
            DespawnRecursive { entity, warn: true }.apply(commands);
        }
    }
}
