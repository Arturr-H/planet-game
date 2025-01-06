/* Imports */
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::TAU;
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
    pub fn generate_foliage_positions(probability_multiplier: f64, seed: u32) -> Vec<f32> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
        let mut positions = Vec::new();
        let perlin = Perlin::new(seed);
        let trees = 100;

        for i in 0..trees {
            let value = (perlin.get([
                (i as f64 / (trees as f64 * 0.8)) * 5.123512,
                (i as f64 / (trees as f64 * 1.25)) * 3.123512,
            ]) + 1.0) / 2.0;

            if rng.gen_bool(value.powi(2) * probability_multiplier) {
                positions.push(i as f32 / trees as f32 * TAU);
            }
        }
    
        positions
    }
}

pub struct FoliagePlugin;
impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TreePlugin);
    }
}
