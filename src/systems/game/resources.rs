
/// The resources that can be found on a planet
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanetResource {
    Wood,
}

/// The resources that the player has
#[derive(Default, Debug)]
pub struct Resources {
    pub wood: usize,
}

impl Resources {
    /// Adds a resource to the player
    pub fn add(&mut self, resource: PlanetResource, amount: usize) {
        match resource {
            PlanetResource::Wood => self.wood += amount,
        }
    }

    /// Removes a resource from the player
    pub fn remove(&mut self, resource: PlanetResource, amount: usize) {
        match resource {
            PlanetResource::Wood => self.wood -= amount,
        }
    }
}
