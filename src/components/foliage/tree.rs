/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;
use crate::{functional::damageable::Damageable, systems::game::PlanetResource};
use super::animation::WindSway;

/// Gives wood when destroyed, will regrow after a while
#[derive(Component)]
pub struct Tree;

impl Tree {
    pub fn spawn(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>, transform: Transform) {
        let mut rng = rand::thread_rng();

        commands.spawn((
            transform,
        )).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: asset_server.load(format!("foliage/birch/0{}.png", rng.gen_range(0..4))),
                    anchor: Anchor::BottomCenter,
                    flip_x: rng.gen_bool(0.5),
                    ..default()
                },
                
                WindSway::new(),
                Damageable::new(50.0, Some((PlanetResource::Wood, rng.gen_range(2..5)))),
            ))
            .observe(Damageable::on_clicked);
        });
    }
}
