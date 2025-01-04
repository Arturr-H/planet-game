use bevy::utils::HashMap;

/// The resources that can be found on a planet
/// 
/// * Important: Don't forget to update the `RESOURCE_TYPES` constant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlanetResource {
    Wood,
}

/// The resources that the player has
#[derive(Debug)]
pub struct Resources {
    map: HashMap<PlanetResource, usize>,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (PlanetResource::Wood, 0),
            ]),
        }
    }
}

impl Resources {
    pub fn get(&self, resource: PlanetResource) -> usize {
        self.map[&resource]
    }

    /// Adds a resource to the player
    pub fn add(&mut self, resource: PlanetResource, amount: usize) {
        *self.map.get_mut(&resource).unwrap() += amount;
    }

    /// Removes a resource from the player
    pub fn remove(&mut self, resource: PlanetResource, amount: usize) {
        *self.map.get_mut(&resource).unwrap() -= amount;
    }
}
