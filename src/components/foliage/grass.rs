/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;

#[derive(Component)]
pub struct Grass;

impl Grass {
    pub fn spawn(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>, transform: Transform) {
        let mut rng = rand::thread_rng();
        let x_offset = rng.gen_range(-1.5..1.5);
        let y_offset = rng.gen_range(-1.5..1.5);

        commands.spawn((
            transform.with_translation(transform.translation + Vec3::new(x_offset, y_offset, 0.0)),
            InheritedVisibility::VISIBLE,
        )).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load(Self::texture()),
                    anchor: Anchor::BottomCenter,
                    flip_x: rng.gen_bool(0.5),
                    ..default()
                },
                Grass,
            ));
        });
    }
    fn texture() -> String {
        format!("foliage/grass/grass.png")
    }
}
