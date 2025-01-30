/* Imports */
use bevy::{prelude::*, utils::HashMap};
use crate::{camera::UI_LAYERS, utils::color::hex};

/* Constants */
#[derive(Component)]
pub struct InventorySlot {
    id: usize,
}

impl InventorySlot {
    pub fn spawn(parent: &mut ChildBuilder, id: usize) -> () {
        parent.spawn((
            InventorySlot { id },
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                
                ..default()
            },
            // BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(hex!("#232323")),
        ));
    }
}
