
use bevy::{prelude::*, sprite::Anchor};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::systems::traits::GenericPointOfInterest;

#[derive(Component, Clone, Copy, Debug)]
pub struct Copper;

impl GenericPointOfInterest for Copper {
    fn spawn(&self,
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        transform: Transform,
    ) -> Entity {
        let mut rng = rand::thread_rng();
        let texture = rng.gen_range(0..2);

        commands.spawn((
            transform,
            InheritedVisibility::VISIBLE,
        )).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load(Self::texture(texture)),
                    anchor: Anchor::BottomCenter,
                    flip_x: rng.gen_bool(0.5),
                    ..default()
                },
                Copper,
            ));
        }).id()
    }
}

impl Copper {
    fn texture(variation: u8) -> String {
        format!("foliage/resource/copper/0{}.png", variation)
    }
}
