/* Imports */
use bevy::prelude::*;
use crate::{components::planet::Planet, systems::game::PlanetResource};

#[enum_delegate::register]
pub trait GenericPointOfInterest {
    /// Spawn logic (bevy)
    fn spawn(
        &self,
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        transform: Transform,
    ) -> ();
}
