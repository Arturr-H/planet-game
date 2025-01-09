/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::{functional::damageable::Damageable, systems::game::PlanetResource, utils::color::hex};
use super::animation::WindSway;

/* Constants */
const MAX_TREE_AGE: u8 = 3;

/// Gives wood when destroyed, will regrow after a while
#[derive(Component)]
pub struct Tree {
    /// Stage 0 = sapling, 1 = young tree, ...
    /// TODO: Harvesting trees in a later age yields more wood
    pub age: u8,
}

impl Tree {
    pub fn spawn(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>, game_seed: u64, transform: Transform) {
        let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
        let initial_age = rng.gen_range(0..1);

        commands.spawn((
            transform,
            InheritedVisibility::VISIBLE,
        )).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load(Self::texture(initial_age)),
                    anchor: Anchor::BottomCenter,
                    flip_x: rng.gen_bool(0.5),
                    ..default()
                },
                Tree { age: initial_age },
                WindSway::new(),
                Damageable::new(
                    50.0,
                    Some((PlanetResource::Wood, rng.gen_range(8..15))),
                    |w: &mut World| {
                        w.run_system_cached(Self::callback).unwrap();
                    }
                ),
            ))
            .observe(Damageable::on_clicked);
        });
    }
    fn texture(age: u8) -> String {
        format!("foliage/birch/0{}.png", age)
    }
    fn callback(mut _commands: Commands, _query: Query<&Transform>) -> () {
        // commands.spawn((
        //     Sprite {
        //         custom_size: Some(Vec2::new(100.0, 100.0)),
        //         color: hex!("#ff0000"),
                
        //         ..default()
        //     },
        //     Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        // ));
        // println!("CALLBACK WORKED");
        // for transform in query.iter() {
        //     println!("CALLBACK WORKEDCALLBACK WORKEDCALLBACK WORKEDCALLBACK WORKED {:?}", transform.translation);
        // }
    }

    /// Increase age
    fn increase_age(&mut self) -> () {
        self.age = (self.age + 1).min(MAX_TREE_AGE);
    }

    /// Every game tick
    fn tick(
        mut query: Query<(&mut Tree, &mut Sprite)>,
        asset_server: Res<AssetServer>,
    ) {
        for (mut tree, mut sprite) in query.iter_mut() {
            if rand::random::<f32>() < 0.01 {
                tree.increase_age();
                sprite.image = asset_server.load(Self::texture(tree.age))
            }
        }
    }
}

pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, Tree::tick);
    }
}
