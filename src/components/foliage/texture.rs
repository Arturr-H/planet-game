/* Imports */
use bevy::prelude::*;

/* Constants */

/// FoliageTexture component
#[derive(Resource)]
pub struct FoliageTextures {
    handles: Vec<(FoliageType, Handle<Image>)>,
}

#[derive(Clone)]
pub enum FoliageType {
    Birch, BigRock
}
const FOLIAGE_TYPES: &[FoliageType] = &[FoliageType::Birch, FoliageType::BigRock];
//, FoliageType::BigRock];

pub struct FoliageTexturePlugin;
impl Plugin for FoliageTexturePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, FoliageTextures::load_textures);
    }
}

impl FoliageTextures {
    /// Setup FoliageTexture handles
    pub fn load_textures(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let mut handles = Vec::new();
        for foliage in FOLIAGE_TYPES {
            for i in 0..foliage.count() {
                let path = foliage.get_path();
                let texture_path = format!("../assets/foliage/{path}{:02}.png", i);
                handles.push((foliage.clone(), asset_server.load(texture_path)));
            }
        }
        commands.insert_resource(FoliageTextures { handles });
    }

    /// Get a random FoliageTexture handle
    pub fn get_random(&self) -> (FoliageType, Handle<Image>) {
        let index = rand::random::<usize>() % self.handles.len();
        self.handles[index].clone()
    }
}

impl FoliageType {
    /// Get path name of folder
    pub fn get_path(&self) -> &str {
        match self {
            FoliageType::Birch => "birch/",
            FoliageType::BigRock => "rock/big/",
        }
    }

    /// Get amount of textures in texture folder
    pub fn count(&self) -> usize {
        match self {
            FoliageType::Birch => 4,
            FoliageType::BigRock => 6,
        }
    }

    /// How much we multiply the distance
    /// from the center of the planet with
    pub fn offset_factor(&self) -> f32 {
        match self {
            FoliageType::Birch => 0.93,
            FoliageType::BigRock => 0.88,
        }
    }
}
