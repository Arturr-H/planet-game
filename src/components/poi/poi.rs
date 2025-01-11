use std::f32::consts::{PI, TAU};

/* Imports */
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use super::{stone::Stone, tree::Tree};
use crate::{components::planet::Planet, systems::traits::GenericPointOfInterest};

/// Some point of interest on the planet, like a stone or a tree.
/// POI:s are often something that can be interacted with via e.g
/// machines or events. 
#[derive(Clone, Copy, Debug)]
pub struct PointOfInterest {
    /// The same as the position index for tiles. Tiles and POI:s
    /// can share the same position index, like drills can be placed
    /// on top of a stone POI to drill it (or close to it).
    pub position_index: usize,

    /// The type of POI, like stone, tree, etc.
    pub poi_type: PointOfInterestType,
}

#[enum_delegate::implement(GenericPointOfInterest)]
#[derive(Clone, Copy, Debug)]
pub enum PointOfInterestType {
    Stone(Stone),
    Tree(Tree),
}

impl PointOfInterest {
    pub fn new(position_index: usize, poi_type: PointOfInterestType) -> Self {
        Self { position_index, poi_type }
    }
    pub fn spawn_multiple(poi_type: PointOfInterestType) -> PointOfInterestBuilder {
        PointOfInterestBuilder::new(poi_type)
    }

    /// Returns a vec of the position indices that a POI will occupy.
    /// `probability` needs to be between 0.0 and 1.0. 
    fn generate_position_indices(planet: &Planet, probability: f32) -> Vec<usize> {
        let noise = Perlin::new(planet.seed);
        let mut rng = ChaCha8Rng::seed_from_u64(planet.seed as u64);
        let mut indices = Vec::new();

        for i in 0..planet.tile_places() {
            let angle = i as f32 / planet.tile_places() as f32 * TAU;
            let x = angle.cos();
            let y = angle.sin();
            let noise_value = (noise.get([x as f64, y as f64]) + 1.0) / 2.0;
            if rng.gen_bool(probability as f64 * noise_value) {
                indices.push(i);
            }
        }

        indices
    }
}

pub struct PointOfInterestBuilder {
    poi_type: PointOfInterestType,
    z_index: f32,
    origin_offset: f32,
    probability: f32,
}
impl PointOfInterestBuilder {
    pub fn new(poi_type: PointOfInterestType) -> Self {
        Self {
            poi_type,
            z_index: 0.0,
            origin_offset: 0.0,
            probability: 0.0,
        }
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self { self.z_index = z_index; self }
    pub fn with_origin_offset(mut self, origin_offset: f32) -> Self { self.origin_offset = origin_offset; self }
    pub fn with_probability(mut self, probability: f32) -> Self { self.probability = probability; self }

    /// Spawns POI:s & registers them in the planet.
    pub fn spawn_all(
        &self,
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        planet: &mut Planet,
    ) {
        assert!(self.probability >= 0.0 && self.probability <= 1.0, "Probability must be between 0.0 and 1.0");
        for position_index in PointOfInterest::generate_position_indices(planet, self.probability) {
            let z = self.z_index + rand::random::<f32>() * 0.025 - 0.0125;
            let transform = planet.index_to_transform(position_index, self.origin_offset, z);
            let new_poi = PointOfInterest { position_index, poi_type: self.poi_type };
            
            match planet.points_of_interest.get_mut(&position_index) {
                Some(e) => e.push(new_poi),
                None => {
                    planet.points_of_interest.insert(position_index, vec![new_poi]);
                }
            }

            self.poi_type.spawn(commands, asset_server, transform);
        }
    }
}
