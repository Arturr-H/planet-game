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
            BackgroundColor(hex!("#ff00ff")),
        ));
    }

    fn on_drop_item(
        interaction_q: Query<&Interaction, (Changed<Interaction>, With<InventorySlot>)>,
        // mut slot: Query<&mut InventorySlot>,
    ) -> () {
        // for interaction in interaction_q {
            // match interaction {
            //     Interaction:: { entity, .. } => {
            //         if let Ok(slot) = interaction_q.get(entity) {
            //             InventorySlot::drop_item(entity, slot);
            //         }
            //     }
            //     _ => {}
            // }
        // }
        // if let Ok(mut slot) = slot.get_single_mut() {
            println!("Dropped item on slot {}", "slot.id");
        // }
    }
}

pub struct InventorySlotPlugin;
impl Plugin for InventorySlotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, InventorySlot::on_drop_item);
    }
}
