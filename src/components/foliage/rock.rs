/* Imports */
use bevy::{prelude::*, sprite::Anchor, utils::HashMap};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const ROCK_VARIANTS: [(usize, &'static str); 2] = [
    (30, "foliage/rock/flat/"),
    (20, "foliage/rock/tall/"),
];

#[derive(Component)]
pub struct Rock;

impl Rock {
    pub fn spawn(
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        transform: Transform
    ) {
        let mut rng = rand::thread_rng();//ChaCha8Rng::seed_from_u64(game_seed);
        let x_offset = rng.gen_range(-1.5..1.5);
        let y_offset = rng.gen_range(-1.5..1.5);

        let (count, path) = ROCK_VARIANTS.choose(&mut rng).unwrap();
        let texture_path = format!("{}{}.png", path, rng.gen_range(0..*count));

        commands.spawn((
            transform.with_translation(transform.translation + Vec3::new(x_offset, y_offset, -0.1)),
            InheritedVisibility::VISIBLE,
        )).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load(texture_path),
                    anchor: Anchor::BottomCenter,
                    flip_x: rng.gen_bool(0.5),
                    ..default()
                },
                Rock,
            ));
        });
    }
    fn texture(variant: u8) -> String {
        format!("foliage/rock/small/0{variant}.png")
    }
}
