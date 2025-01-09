/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Component)]
pub struct Stone;

impl Stone {
    pub fn spawn(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>, game_seed: u64, transform: Transform) {
        let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
        let texture = rng.gen_range(0..6);

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
                Stone,
            ));
        });
    }
    fn texture(age: u8) -> String {
        format!("foliage/rock/big/0{}.png", age)
    }
}
