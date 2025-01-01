/* Imports */
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::TAU; // 2π

/* Constants */
const FOLIAGE_MIN_ANGLE_DISTANCE: f32 = 0.25;

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
    pub fn generate_foliage_positions(count: usize) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::new();
        let mut tries = 0;

        while positions.len() < count {
            if tries > 10_000 {
                break;
            }
            let angle = rng.gen_range(0.0..TAU);
            let weight = (angle.sin().abs() + 0.1).powf(2.0);
    
            if rng.gen::<f32>() < weight {
                if positions.iter().all(|&a| Self::angular_distance(a, angle) >= FOLIAGE_MIN_ANGLE_DISTANCE) {
                    positions.push(angle);
                }
            }

            tries += 1;
        }
    
        positions
    }

    #[inline]
    fn angular_distance(a1: f32, a2: f32) -> f32 {
        let diff = (a1 - a2).abs() % TAU;
        diff.min(TAU - diff)
    }
}
