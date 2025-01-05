/* Imports */
use bevy::prelude::*;

/* Constants */
pub const GAME_TICK_HZ: f64 = 20.0;

pub struct GameTickPlugin;
impl Plugin for GameTickPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_hz(GAME_TICK_HZ));
    }
}
