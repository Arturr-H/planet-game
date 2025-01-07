/* Imports */
use bevy::prelude::*;

/// Keeps track of slots that are clicked
#[derive(Resource, Default, Debug)]
pub struct SlotCablePlacementResource {
    /// (id, entity)
    active: Option<(usize, Entity)>,

    /// The transform of the start entity. (not global)
    /// will be compared to the global transform of the
    /// end entity to determine if the cable is valid
    /// (no longer than MAX_CABLE_LENGTH)
    pub start_entity_pos: Vec2,
}

impl SlotCablePlacementResource {
    pub fn set_active(&mut self, id: usize, entity: Entity, transform: Vec3) {
        self.active = Some((id, entity));
        self.start_entity_pos = transform.truncate().xy();
    }
    pub fn reset(&mut self) { self.active = None; }
    pub fn active(&self) -> Option<(usize, Entity)> { self.active }
}
fn order(a: usize, b: usize) -> (usize, usize) {
    if a > b {
        (b, a)
    }else {
        (a, b)
    }
}
