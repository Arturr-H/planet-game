use super::GenericTile;

/// All tiles that can store energy
pub trait EnergyStorage: GenericTile {
    /// Maximum amount of energy that can be stored
    fn capacity(&self) -> f32 { 50.0 }

    /// Current amount of energy stored
    fn stored(&self) -> f32;

    /// Add energy to the storage
    fn add_energy(&mut self, amount: f32);
}
