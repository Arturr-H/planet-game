/* Imports */
use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, sprite::Material2d};

/* Constants */
const SHADER_PATH: &str = "shaders/flag/flag.wgsl";

#[derive(AsBindGroup, ShaderType, Debug, Clone, Copy)]
pub struct FlagMaterialParams {
    pub color: LinearRgba,
    pub time: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub speed: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlagMaterial {
    #[uniform(0)]
    pub params: FlagMaterialParams,
}


impl Material2d for FlagMaterial {
    fn fragment_shader() -> ShaderRef { SHADER_PATH.into() }
}
