use bevy::{
    input::mouse::MouseWheel, prelude::*, render::{
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

use crate::{utils::color::hex, RES_HEIGHT, RES_WIDTH};

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
// pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(0);
/// Render layers for UI rendering.
pub const UI_LAYERS: RenderLayers = RenderLayers::layer(1);

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(hex!("#87CEEB")))
            .add_systems(Startup, initialize);
            // .add_systems(Update, fit_canvas);
    }
}

pub struct CameraDebugPlugin;
impl Plugin for CameraDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, debug_control);
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

/// Set up cameras and canvas.
pub fn initialize(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>
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

    let image_handle = images.add(canvas);

    // commands.spawn((
    //     Camera2d,
    //     Camera {
    //         // render before the "main pass" camera
    //         order: -1,
    //         target: RenderTarget::Image(image_handle.clone()),
    //         ..default()
    //     },
    //     Msaa::Off,
    //     InGameCamera,
    //     PIXEL_PERFECT_LAYERS,
    // ));

    // commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
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

/// Scales camera projection to fit the window (integer
/// multiples only) on window resize.
// pub fn fit_canvas(
//     mut resize_events: EventReader<WindowResized>,
//     mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
// ) {
//     for event in resize_events.read() {
//         let h_scale = event.width / RES_WIDTH as f32;
//         let v_scale = event.height / RES_HEIGHT as f32;
//         let mut projection = projections.single_mut();
//         projection.scale = 1. / h_scale.min(v_scale).round();
//     }
// }

/// Zooms the camera in and out using the mouse wheel.
pub fn debug_control(
    mut query: Query<&mut OrthographicProjection, With<OuterCamera>>,
    mut scroll: EventReader<MouseWheel>,
    kb: Res<ButtonInput<KeyCode>>
) {
    for event in scroll.read() {
        for mut projection in query.iter_mut() {
            projection.scale *= 1. + event.y * -0.0002;
        }
    }

    if kb.just_pressed(KeyCode::Backspace) {
        for mut projection in query.iter_mut() {
            projection.scale = 1.;
        }
    }
    if kb.pressed(KeyCode::KeyL) {
        for mut projection in query.iter_mut() {
            projection.scale *= 1.01;
        }
    }else if kb.pressed(KeyCode::KeyO) {
        for mut projection in query.iter_mut() {
            projection.scale *= 0.99;
        }
    }
}
