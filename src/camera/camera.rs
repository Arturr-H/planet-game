use std::f32::consts::PI;

const CAMERA_DAMPING: f32 = 1.0; // 1 = no damping 2 = pretty smooth, less than 1 = do not


use bevy::{
    core_pipeline::post_process, input::mouse::{MouseMotion, MouseWheel}, prelude::*, render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d,
            TextureDescriptor,
            TextureDimension,
            TextureFormat,
            TextureUsages
        },
        view::RenderLayers
    }, window::WindowResized
};
use crate::{components::{planet::{CameraPlanetRotation, Planet, PlayerPlanet}, player::player::Player}, systems::game::GameState, utils::color::hex, RES_HEIGHT, RES_WIDTH};
use super::post_processing::{PostProcessPlugin, PostProcessSettings};

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
// pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(0);
/// Render layers for UI rendering.
pub const UI_LAYERS: RenderLayers = RenderLayers::layer(1);

const CAMERA_ELEVATION: f32 = 50.0;

#[derive(Resource)]
pub struct CameraSettings {
    pub elevation: f32,
    pub is_panning: bool,

    pub total_delta: Vec2,
    pub start_transform: Transform,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            elevation: CAMERA_ELEVATION,
            is_panning: false,
            total_delta: Vec2::ZERO,
            start_transform: Transform::default(),
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PostProcessPlugin)
            .insert_resource(ClearColor(hex!("#000000")))
            .insert_resource(CameraSettings::default())
            .add_systems(Startup, Self::initialize)
            .add_systems(Update, Self::update_camera_scale);
            // .add_systems(Update, fit_canvas);
    }
}

pub struct CameraDebugPlugin;
impl Plugin for CameraDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, Self::debug_control.after(Player::update));
    }
}

// Camera that renders the pixel-perfect world to the [`Canvas`].
// #[derive(Component)]
// pub struct InGameCamera;

/// This camera primarily is used to render the pixel-perfect
/// [`Canvas`] to the screen. But this camera can also render
/// other high-resolution entities like UI.
#[derive(Component)]
pub struct OuterCamera;

/// Renders pixel perfect UI
#[derive(Component)]
pub struct UiCamera;

/// Rendered to the high-resolution camera. The pixel-perfect
/// game view is rendered to this Canvas.
#[derive(Component)]
struct Canvas;

impl CameraPlugin {
    /// Set up cameras and canvas.
    pub fn initialize(
        mut commands: Commands,
        mut images: ResMut<Assets<Image>>,
        player_q: Query<&Transform, With<Player>>,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
    ) -> () {
        let canvas_size = Extent3d {
            width: RES_WIDTH as u32,
            height: RES_HEIGHT as u32,
            ..default()
        };

        let mut canvas = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: canvas_size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };

        canvas.resize(canvas_size);

        let _image_handle = images.add(canvas);

        commands.spawn((
            Camera2d,
            Msaa::Off,
            OuterCamera,
            HIGH_RES_LAYERS,

            PostProcessSettings {
                base_pixel_size: 1.0,
                screen_height: 0.0,
                screen_width: 0.0,
                camera_scale: 1.0,
                ..default()
            },
        ));

        commands.spawn((
            Camera2d,
            IsDefaultUiCamera,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Msaa::Off,
            UiCamera,
            UI_LAYERS,
        ));
    }

    pub fn update_camera_scale(
        mut resize_events: EventReader<WindowResized>,
        mut settings: Query<&mut PostProcessSettings, With<OuterCamera>>,
    ) {
        for event in resize_events.read() {
            let mut settings = settings.single_mut();
            settings.screen_width = event.width;
            settings.screen_height = event.height;
        }
    }

    fn update_camera_transform(
        planet: &Planet,
        radians: f32,
        camera_transform: &mut Transform,
        elevation: f32,
    ) -> () {
        // let planet = planet_q.single();
        // let camera_radians = Planet::normalize_radians(radians + PI / 2.0);
        let camera_radians = Planet::normalize_radians(radians);
        let (translation, surface_angle) = planet.radians_to_radii(camera_radians, elevation);
        let mul = (CAMERA_DAMPING - 1.0) * (planet.radius + elevation);
        camera_transform.translation = Vec3::new(
            (translation.x + mul * camera_radians.cos()) / CAMERA_DAMPING,
            (translation.y + mul * camera_radians.sin()) / CAMERA_DAMPING,
            camera_transform.translation.z
        );
        camera_transform.rotation = Quat::from_rotation_z(Planet::normalize_radians(surface_angle + PI));
    }
}

impl CameraDebugPlugin {
    pub fn debug_control(
        mut camera_transform_q: Query<(&mut OrthographicProjection, &mut PostProcessSettings, &mut Transform), With<OuterCamera>>,
        mut scroll: EventReader<MouseWheel>,
        kb: Res<ButtonInput<KeyCode>>,
        mut camera_settings: ResMut<CameraSettings>,
        mut camera_rotation: ResMut<CameraPlanetRotation>,
        planet_q: Query<&Planet, With<PlayerPlanet>>,
        mut mouse_motion: EventReader<MouseMotion>,
        mouse: Res<ButtonInput<MouseButton>>,
        player_q: Query<&Player, With<Player>>,
    ) {
        let pan_speed = 1.0;
        let mut pan_delta = Vec2::ZERO;
        
        for event in mouse_motion.read() {
            pan_delta += event.delta;
        }

        if mouse.pressed(MouseButton::Right) {
            if let Ok((projection, _, mut transform)) = camera_transform_q.get_single_mut() {
                if let Ok(planet) = planet_q.get_single() {
                    let rotation = transform.rotation;
                    
                    let world_delta = rotation * Vec3::new(
                        -pan_delta.x * projection.scale * pan_speed,
                        pan_delta.y * projection.scale * pan_speed,
                        0.0
                    );

                    transform.translation += world_delta;

                    let pos = transform.translation.truncate();
                    let pos_angle = pos.y.atan2(pos.x);
                    let (surface_pos, surface_angle) = planet.radians_to_radii(pos_angle, 0.0);
                    let surface_radius = surface_pos.length();
                    let current_elevation = pos.length() - surface_radius;
                    let clamped_elevation = current_elevation.clamp(-5.0, 120.0);

                    if clamped_elevation != current_elevation {
                        let direction = pos.normalize();
                        let new_pos = direction * (surface_radius + clamped_elevation);
                        transform.translation = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
                    }

                    transform.rotation = Quat::from_rotation_z(
                        Planet::normalize_radians(surface_angle + PI)
                    );
                }
            }
            camera_settings.is_panning = true;
        } else if camera_settings.is_panning{
            if let Ok((_, _, transform)) = camera_transform_q.get_single() {
                if let Ok(planet) = planet_q.get_single() {
                    
                    let pos = transform.translation.truncate();
                    let pos_angle = pos.y.atan2(pos.x);
                    let (translation, _) = planet.radians_to_radii(pos_angle, 0.0);

                    camera_settings.elevation = (pos.length() - translation.length()).clamp(-5.0, 120.0);
                    camera_rotation.radians = pos_angle;
                }
            }
            camera_settings.is_panning = false;
        } else {
            if let Ok(player) = player_q.get_single() {
                if let Ok((_, _, mut transform)) = camera_transform_q.get_single_mut() {
                    if let Ok(planet) = planet_q.get_single() {
                        let target_rotation = player.radians;

                        let rotation_delta = (target_rotation - camera_rotation.radians + PI).rem_euclid(2.0 * PI) - PI;
                        // let rotation_delta = (target_rotation - camera_rotation.radians);
                        camera_rotation.radians += rotation_delta * 0.1;

                        CameraPlugin::update_camera_transform(
                            planet,
                            camera_rotation.radians,
                            &mut transform,
                            camera_settings.elevation,
                        );
                    }
                }
            }
        }

        for event in scroll.read() {
            for (mut projection, mut settings, _) in camera_transform_q.iter_mut() {
                projection.scale *= 1.0 + event.y * -0.04;
                // if settings.camera_scale > 2.0 { return }
                settings.camera_scale = projection.scale;
            }
        }

        if kb.just_pressed(KeyCode::Backspace) {
            for (mut projection, mut settings, _) in camera_transform_q.iter_mut() {
                projection.scale = 1.;
                settings.camera_scale = projection.scale;
            }
        }
        if kb.pressed(KeyCode::KeyL) {
            for (mut projection, mut settings, _) in camera_transform_q.iter_mut() {
                println!("Scale: {}", settings.camera_scale);
                if settings.camera_scale > 10.0 { return }
                projection.scale *= 1.01;
                settings.camera_scale = projection.scale;
            }
        }else if kb.pressed(KeyCode::KeyO) {
            for (mut projection, mut settings, _) in camera_transform_q.iter_mut() {
                projection.scale *= 0.99;
                settings.camera_scale = projection.scale;
            }
        }
    }
}
