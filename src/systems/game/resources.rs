use bevy::utils::HashMap;

/// The resources that can be found on a planet
/// 
/// * Important: Don't forget to update the `RESOURCE_TYPES` constant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlanetResource {
    Wood,
    Stone,
}

/// The resources that the player has
#[derive(Debug, Clone)]
pub struct PlanetResources {
    map: HashMap<PlanetResource, usize>,
}

impl Default for PlanetResources {
    fn default() -> Self {
        Self {
            map: HashMap::from([
                (PlanetResource::Wood, 150),
                (PlanetResource::Stone, 150),
            ]),
        }
    }
}

impl PlanetResources {
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
    pub fn has(&self, resource: PlanetResource, amount: usize) -> bool {
        self.get(resource) >= amount
    }

    /// Returns a user-friendly error message if player has
    /// insufficient amount of materials (e.g buying a powerpole)
    pub fn try_spend(&mut self, resources: Vec<(PlanetResource, usize)>) -> Result<(), String> {
        /* Try spend materials */
        for (resource, cost) in &resources {
            if !self.has(*resource, *cost) {
                let player_has = self.get(*resource);
                let items_left = cost - player_has;
                return Err(format!("Need {items_left}x more {resource:?}"))
            }
        }

        /* Spend */
        for (resource, cost) in resources { self.remove(resource, cost); }
        Ok(())
    }
}
