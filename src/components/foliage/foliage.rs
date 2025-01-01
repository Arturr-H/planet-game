/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;
use crate::{functional::damageable::{DamageEvent, Damageable}, RES_WIDTH};
use super::texture::{self, FoliageTexturePlugin, FoliageTextures};

/// Foliage component
#[derive(Component)]
pub struct Foliage;

pub struct FoliagePlugin;
impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(texture::FoliageTexturePlugin);
    }
}

impl Foliage {
    /// Setup Foliage
    pub fn spawn(commands: &mut ChildBuilder, foliage_textures: &Res<FoliageTextures>, degree: f32) -> () {
        let mut rng = rand::thread_rng();
        let (foliage, handle) = foliage_textures.get_random();
        let offset_factor = foliage.offset_factor();
        let x = degree.cos() * RES_WIDTH / 2.0 * offset_factor;
        let y = degree.sin() * RES_WIDTH / 2.0 * offset_factor;
        let scale = rng.gen_range(0.8..1.2);
        
        commands.spawn((
            Sprite {
                image: handle,
                anchor: Anchor::BottomCenter,
                flip_x: rng.gen_bool(0.5),
                ..default()
            },
            Transform {
                translation: Vec3::new(x, y, -0.5),
                rotation: Quat::from_rotation_z(degree - std::f32::consts::PI / 2.0),
                scale: Vec3::new(scale, scale, 1.0)
            },
            Damageable::new(50.0),
            // On::<bevy_mod_picking::prelude::Pointer<bevy_mod_picking::prelude::Down>>::send_event::<DamageEvent>(),
            Foliage,
        ))
        .observe(Damageable::on_clicked);
    }
}
