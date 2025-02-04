/* Imports */
use bevy::{prelude::*, window::PrimaryWindow};
use crate::{camera::{OuterCamera, UiCamera, UI_LAYERS}, hex};

#[derive(Component)]
pub struct Item {
    pub quantity: usize,
    pub name: String
}
#[derive(Component)]
pub struct ItemPreview;

pub struct SpawnItemPreview { pub item: Item }
impl Command for SpawnItemPreview {
    fn apply(self, world: &mut World) {
        // Item::spawn_preview(world.commands(), self.item);
    }
}
pub struct SpawnItemInSlot { pub item: Item, pub slot: Entity }

impl Item {
    pub fn new(name: String, quantity: usize) -> Self {
        Item { name, quantity }
    }

    /// The sprite which contains the item's image
    /// can't be a Node because transform does not
    /// apply to it.
    pub fn spawn_preview(
        mut commands: Commands,
    ) {
        commands.spawn((
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(hex!("#ff0000")),
            ItemPreview,
            GlobalZIndex(2000),
            UI_LAYERS
        ))
        .observe(Self::on_drag)
        .observe(Self::on_release);
    }

    fn on_drag(
        evt: Trigger<Pointer<Drag>>,
        mut node: Query<&mut Node, With<ItemPreview>>,
    ) -> () {
        if let Ok(mut node) = node.get_single_mut() {
            let Val::Px(left) = node.left else { return };
            let Val::Px(top) = node.top else { return };
            node.left = Val::Px(left + evt.delta.x);
            node.top = Val::Px(top + evt.delta.y);
        }
    }

    fn on_release(
        mut evt: Trigger<Pointer<Up>>,
        mut node: Query<&mut Node, With<ItemPreview>>,
    ) -> () {
        evt.propagate(true);
        println!("{:?}", evt.target);
    }

    pub fn update_preview(
        windows: Query<&Window>,
        camera: Query<(&Camera, &GlobalTransform), With<UiCamera>>,
        mut node: Query<&mut Node, With<ItemPreview>>,
    ) {
        let window = windows.single();
        let (camera, camera_transform) = camera.single();
    
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            for mut transform in &mut node {
                // transform.left = Val::Px(world_position.x);

                // if let Val::Px(left) = transform.left {}
                // transform.rotation = camera_transform.rotation();
            }
        }
    }
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Item::spawn_preview)
            .add_systems(Update, Item::update_preview);
    }
}
