/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::planet::{self, Planet, PlayerPlanet}, utils::color::hex};

/* Constants */

/// Player component
#[derive(Component)]
pub struct Player {
    pub radians: f32,
}

impl Player {
    pub fn setup(
        mut commands: Commands,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
    ) -> () {
        let planet = planet_q.single();
        commands.spawn((
            Player { radians: 0.0 },
            Sprite {
                color: hex!("1e81b0"),
                custom_size: Some(Vec2::new(10.0, 20.0)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            planet.index_to_transform(0, 0.0, 10.0, 1)
        ));
    }
    pub fn update(
        kb: Res<ButtonInput<KeyCode>>,
        mut player_q: Query<(&mut Transform, &mut Player), With<Player>>,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
    ) -> () {
        let planet = planet_q.single(); 
        if kb.pressed(KeyCode::KeyA) {
            for (mut transform, mut player) in player_q.iter_mut() {
                player.radians += 0.001;

                let new_transform = planet.radians_to_transform(player.radians, 0.0, 10.0);
                transform.translation = new_transform.translation;
                transform.rotation = new_transform.rotation;
            }
        }
        if kb.pressed(KeyCode::KeyD) {
            for (mut transform, mut player) in player_q.iter_mut() {
                player.radians -= 0.001;

                let new_transform = planet.radians_to_transform(player.radians, 0.0, 10.0);
                transform.translation = new_transform.translation;
                transform.rotation = new_transform.rotation;
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Player::setup.after(Planet::setup))
            .add_systems(Update, Player::update);
    }
}
