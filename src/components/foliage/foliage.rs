/* Imports */
use bevy::{prelude::*, transform};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::TAU;
use crate::components::planet::Planet;

use super::tree::TreePlugin;

/* Constants */

/// This is just a wrapper for methods
/// that are used for all types of foliage
/// 
/// All methods should be static
pub struct Foliage;

impl Foliage {
    /// The planet is a sphere, so the foliage
    /// will be spread out between radians of 0
    /// 2π. This method will generate kind of a 
    /// noise-map where some areas will have more
    /// foliage than others. Returns a vector of
    /// radians where the foliage should be placed
    /// 
    /// (otherwise it looks so uniform and non-natural)
    /// I don't care about efficiency because it's
    /// called once in a startup function.
    pub fn generate_foliage_positions(
        probability_multiplier: f64, points: usize, seed: u32,
        spawn_function: fn(&mut ChildBuilder, &Res<AssetServer>, Transform),
        asset_server: &Res<AssetServer>, commands: &mut ChildBuilder,
        planet: &Planet, z_index: f32
    ) -> () {
        let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
        let perlin = Perlin::new(seed);

        for i in 0..points {
            let value = (perlin.get([
                (i as f64 / (points as f64 * 0.8)) * 5.123512,
                (i as f64 / (points as f64 * 1.25)) * 3.123512,
            ]) + 1.0) / 2.0;
            let degree = (i as f32 / points as f32) * TAU;

            if rng.gen_bool(value.powi(2) * probability_multiplier) {
                let origin_offset = -6.0 - rng.gen_range(0.0..5.0);
                let transform = planet.radians_to_transform(degree, origin_offset, -0.1);
                let scale = rng.gen_range(0.9..1.1);
                (spawn_function)(
                    commands,
                    asset_server,
                    transform
                        .with_scale(Vec3::new(scale, scale, 1.0))
                        .with_translation(transform.translation
                            + Vec3::new(0.0, 0.0, z_index + rng.gen_range(-0.01..0.01))),
                );
            }
        }
    }
}

pub struct FoliagePlugin;
impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TreePlugin);
    }
}
