/* Imports */
use bevy::prelude::*;

use crate::utils::color::hex;

/* Constants */

/// DebugComponent displays a square with some text
#[derive(Component)]
pub struct DebugComponent;

impl DebugComponent {
    pub fn setup(
        commands: &mut ChildBuilder,
        text: &str,
        transform: Transform,
    ) -> () {
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                color: hex!("#f99a3a"),
                ..Default::default()
            },
            transform,
            DebugComponent,
        ));
        commands.spawn((
            Text2d(text.to_string()),
            transform.with_translation(transform.translation + Vec3::new(0.0, 0.0, 1.0)),
        ));
    }
    pub fn update() -> () {}
}
