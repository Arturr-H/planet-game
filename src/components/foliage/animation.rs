/* Imports */
use bevy::prelude::*;
use rand::Rng;

/// Foliage animation. 
/// 
/// (a, b) where a is the amplitude and b is
/// the offset of the wind swaying.
/// f(x) = a*sin(b + x)
#[derive(Component)]
pub struct WindSway(f32, f32);

impl WindSway {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self(
            rng.gen_range(0.7..1.3),
            rng.gen_range(0.0..5.0)
        )
    }
    pub fn update(
        time: Res<Time>,
        mut query: Query<(&WindSway, &mut Transform)>,
    ) {
        let time = time.elapsed_secs();
        for (sway, mut transform) in query.iter_mut() {
            let amplitude = 0.01 * sway.0;
            let sway = (time * 2.0 + sway.1).sin() * amplitude - amplitude / 2.0;
            transform.rotation = Quat::from_rotation_z(sway);
        }
    }
}

#[derive(Component)]
pub struct Rotate(pub f32);
impl Rotate {
    pub fn update(
        time: Res<Time>,
        mut query: Query<(&Rotate, &mut Transform)>,
    ) {
        let time = time.delta_secs();
        for (rotate, mut transform) in query.iter_mut() {
            transform.rotate(Quat::from_rotation_z(rotate.0 * time));
        }
    }
}

pub struct FoliageAnimationPlugin;
impl Plugin for FoliageAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (WindSway::update, Rotate::update));
    }
}
