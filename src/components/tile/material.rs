use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TileMaterialOutline {
    /// The color of the outline.
    #[uniform(0)]
    pub color: LinearRgba,
    /// The thickness of the outline. Preferred values between 0.01 and 0.005.
    // #[uniform(0)]
    // pub thickness: f32,
    /// The texture to outline.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for TileMaterialOutline {
    fn fragment_shader() -> ShaderRef {
        "shaders/tile_material_outline.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
