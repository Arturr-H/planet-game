use std::time::Duration;

/* Imports */
use bevy::{prelude::*, sprite::Anchor};
use crate::{components::planet::{self, Planet, PlayerPlanet}, utils::color::hex};

/* Constants */

/// Player component
#[derive(Component)]
pub struct Player {
    pub radians: f32,
    pub speed: f32,
}


#[derive(Component)]
pub struct RunAnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct RunAnimationTimer(Timer);



impl Player {
    pub fn setup(
        mut commands: Commands,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    ) -> () {
        let planet = planet_q.single();
        let layout = TextureAtlasLayout::from_grid(UVec2::new(28, 28), 9, 1, None, None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);

        commands.spawn((
            Player { radians: 0.0, speed: 10.0, },
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    index: 0,
                    layout: texture_atlas_layout,
                }),
                anchor: Anchor::BottomCenter,
                image: asset_server.load("../assets/player/player-ball.png"),
                // custom_size: Some(Vec2::new(16.0, 28.0)),

                ..default()
            },
            SpatialListener::new(200.0), //the distance between the "ears"
            planet.index_to_transform(0, 0.0, 10.0, 1),
            RunAnimationIndices { first: 0, last: 8 },
            RunAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
    pub fn update(
        kb: Res<ButtonInput<KeyCode>>,
        mut player_q: Query<(&mut Transform, &mut Player, &mut Sprite, &RunAnimationIndices, &mut RunAnimationTimer), With<Player>>,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
        time: Res<Time>,
    ) -> () {
        let planet = planet_q.single(); 

        for (_, mut player, _, _, mut run_animation_timer) in player_q.iter_mut() {
            if kb.pressed(KeyCode::ShiftLeft) {
                player.speed = 20.0;
                run_animation_timer.set_duration(Duration::from_secs_f32(0.03));
            } else if kb.pressed(KeyCode::ControlLeft) {
                player.speed = 3.0;
                run_animation_timer.set_duration(Duration::from_secs_f32(0.3));
            } else {
                player.speed = 10.0;
                run_animation_timer.set_duration(Duration::from_secs_f32(0.1));
            }
        }
        

        let mut backwards = false;

        if kb.pressed(KeyCode::KeyA) {
            for (mut transform, mut player, mut sprite, _, _) in player_q.iter_mut() {
                player.radians += player.speed / 10000.0;

                let new_transform = planet.radians_to_transform(player.radians, 0.0, 10.0);
                transform.translation = new_transform.translation;
                transform.rotation = new_transform.rotation;
                
                // sprite.flip_x = true;
            }

            backwards = true;
        }
        if kb.pressed(KeyCode::KeyD) {
            for (mut transform, mut player, mut sprite, _, _) in player_q.iter_mut() {
                player.radians -= player.speed / 10000.0;

                let new_transform = planet.radians_to_transform(player.radians, 0.0, 10.0);
                transform.translation = new_transform.translation;
                transform.rotation = new_transform.rotation;

                // sprite.flip_x = false;
            }

            backwards = false;
        }
        if kb.pressed(KeyCode::KeyD) ^ kb.pressed(KeyCode::KeyA) {
            
            Self::animate_run(time, player_q, backwards);
        }
        // else {
        //     for (_, _, mut sprite, indices, _) in &mut player_q {
        //         if let Some(atlas) = &mut sprite.texture_atlas {
        //             atlas.index = indices.first;
        //         }
        //     }
        // }
    }

    fn animate_run(
        time: Res<Time>,
        mut player_q: Query<(
            &mut Transform,
            &mut Player,
            &mut Sprite,
            &RunAnimationIndices,
            &mut RunAnimationTimer,
        ), With<Player>>,
        backwards: bool,
    ) {
        for (_, _, mut sprite, indices, mut timer) in &mut player_q {
            timer.tick(time.delta());
    
            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    let next_index = if backwards {
                        if atlas.index == indices.first {
                            indices.last
                        } else {
                            atlas.index - 1
                        }
                    } else {
                        if atlas.index == indices.last {
                            indices.first
                        } else {
                            atlas.index + 1
                        }
                    };
                    
                    atlas.index = next_index;
                }
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
