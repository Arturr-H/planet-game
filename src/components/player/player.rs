/* Imports */
use bevy::prelude::*;
use crate::{components::planet::{self, Planet, PlayerPlanet}, utils::color::hex};

/* Constants */

/// Player component
#[derive(Component)]
pub struct Player;

impl Player {
    pub fn setup(
        mut commands: Commands,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
    ) -> () {
        let planet = planet_q.single();
        commands.spawn((
            Player,
            Sprite {
                color: hex!("ff0000"),
                custom_size: Some(Vec2::splat(50.0)),
                ..default()
            },
            planet.index_to_transform(0, 0.0, 10000.0, 1)
        ));
    }
    pub fn update() -> () {}
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Player::setup.after(Planet::setup))
            .add_systems(Update, Player::update);
    }
}
