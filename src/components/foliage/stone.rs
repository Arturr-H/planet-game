/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;

#[derive(Component)]
pub struct Stone;

impl Stone {
    pub fn spawn(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>, transform: Transform) {
        let mut rng = rand::thread_rng();
        let texture = rng.gen_range(0..6);

        commands.spawn((
            transform,
        )).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load(Self::texture(texture)),
                    anchor: Anchor::BottomCenter,
                    flip_x: rng.gen_bool(0.5),
                    ..default()
                },
                Stone,
            ));
        });
    }
    fn texture(age: u8) -> String {
        format!("foliage/rock/big/0{}.png", age)
    }
}
