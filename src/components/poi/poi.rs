use std::{f32::consts::{PI, TAU}, mem::discriminant};

/* Imports */
use bevy::{prelude::*, text::cosmic_text::ttf_parser::loca};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use super::{copper::Copper, flag::flag, stone::Stone, tree::Tree};
use crate::{components::{cable::slot::RemoveAllCableSlotHighlightsCommand, planet::Planet}, systems::traits::GenericPointOfInterest, utils::color::hex};

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

    pub entity: Entity,
}

#[enum_delegate::implement(GenericPointOfInterest)]
#[derive(Clone, Copy, Debug)]
pub enum PointOfInterestType {
    Stone(Stone),
    Copper(Copper),
    Tree(Tree),
}

// We only want to compare the type of POI, the content
// of each enum variant is a ZST and doesn't need to be compared.
impl PartialEq for PointOfInterestType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl PointOfInterest {
    pub fn new(position_index: usize, poi_type: PointOfInterestType) -> Self {
        Self { position_index, poi_type, entity: Entity::PLACEHOLDER }
    }
    pub fn spawn_multiple() -> PointOfInterestBuilder {
        PointOfInterestBuilder::new()
    }

    /// Returns a vec of the position indices that a POI will occupy.
    /// `probability` needs to be between 0.0 and 1.0. 
    fn generate_position_indices(planet: &Planet, local_seed: u32, probability: f32) -> Vec<usize> {
        let noise = Perlin::new(planet.seed + local_seed);
        let mut rng = ChaCha8Rng::seed_from_u64((planet.seed + local_seed) as u64);
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
    types: Vec<(PointOfInterestType, f32)>, // type and weight
    z_index: f32,
    origin_offset: f32,
    probability: f32,
    local_seed: u32,
}
impl PointOfInterestBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            z_index: 0.0,
            origin_offset: 0.0,
            probability: 0.0,
            local_seed: 0,
        }
    }

    pub fn add_type(mut self, poi_type: PointOfInterestType, weight: f32) -> Self { self.types.push((poi_type, weight)); self }
    pub fn with_local_seed(mut self, local_seed: u32) -> Self { self.local_seed = local_seed; self }
    pub fn with_z_index(mut self, z_index: f32) -> Self { self.z_index = z_index; self }
    pub fn with_origin_offset(mut self, origin_offset: f32) -> Self { self.origin_offset = origin_offset; self }
    pub fn with_probability(mut self, probability: f32) -> Self { self.probability = probability; self }
    // pub fn with_replacement(mut self, poi_type: PointOfInterestType, probability: f32) -> Self { self.replacements.push((poi_type, probability)); self }

    /// Spawns POI:s & registers them in the planet.
    pub fn spawn_all(
        &self,
        commands: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        planet: &mut Planet,
    ) {
        assert!(self.probability >= 0.0 && self.probability <= 1.0, "Probability must be between 0.0 and 1.0");
        assert!(!self.types.is_empty(), "At least one POI type must be added");

        let position_indices = PointOfInterest::generate_position_indices(planet, self.local_seed, self.probability);

        for position_index in position_indices {
            let total_weight: f32 = self.types.iter().map(|(_, w)| w).sum();
            let mut rng = ChaCha8Rng::seed_from_u64(
                (planet.seed + self.local_seed) as u64 + position_index as u64
            );
            let mut random = rng.gen_range(0.0..total_weight);

            let selected_type = self.types.iter()
                .find(|(_, weight)| {
                    random -= *weight;
                    random <= 0.0
                })
                .map(|(t, _)| *t)
                .unwrap_or_else(|| self.types[0].0);
            
            let z = self.z_index + rand::random::<f32>() * 0.025 - 0.0125;
            let transform = planet.index_to_transform(position_index, self.origin_offset, z, 0);
            let entity = selected_type.spawn(commands, asset_server, transform);
            let new_poi = PointOfInterest { position_index, poi_type: selected_type, entity };
            
            match planet.points_of_interest.get_mut(&position_index) {
                Some(e) => e.push(new_poi),
                None => {
                    planet.points_of_interest.insert(position_index, vec![new_poi]);
                }
            }
        }
    }
}

/// A highlight animation for a point of interest.
#[derive(Component)]
pub struct PointOfInterestHighlight {
    time: f32,
    max_time: f32,
    color: Srgba,
}

impl Default for PointOfInterestHighlight {
    fn default() -> Self {
        Self { time: 0.0, max_time: 0.05, color: Color::WHITE.into() }
    }
}
impl PointOfInterestHighlight {
    pub fn new() -> Self { Self::green() }
    pub fn green() -> Self { Self { color: Color::srgb(0.0, 1.2, 0.0).into(), ..default() } }
    pub fn red() -> Self { Self { color: hex!("#db1a1a").into(), ..default() } }

    pub fn update(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(Entity, &Children, &mut PointOfInterestHighlight)>,
        mut highlight_q: Query<&mut Sprite>,
    ) {
        for (entity, children, mut highlight) in query.iter_mut() {
            highlight.time += time.delta_secs();

            for child in children {
                match highlight_q.get_mut(*child) {
                    Ok(mut sprite) => {
                        let Srgba { red, green, blue, .. }
                            = highlight.color;

                        let sprite_color = sprite.color.to_srgba();
                        sprite.color = Color::srgba(
                            red, green, blue,
                            sprite_color.alpha
                        );
                    },
                    Err(_) => ()
                }
            }

            if highlight.time > highlight.max_time {
                for child in children {
                    match highlight_q.get_mut(*child) {
                        Ok(mut sprite) => sprite.color = Color::WHITE,
                        Err(_) => ()
                    }
                }
                
                commands.queue(RemoveAllCableSlotHighlightsCommand);
                commands.entity(entity).remove::<PointOfInterestHighlight>();
            }
        }
    }
}

pub struct PointOfInterestPlugin;
impl Plugin for PointOfInterestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(flag::FlagPlugin)
            .add_systems(Update, PointOfInterestHighlight::update);
    }
}
