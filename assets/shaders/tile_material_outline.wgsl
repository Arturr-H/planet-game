#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct ColorMaterial {
    color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: ColorMaterial;
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let texture_color = textureSample(base_color_texture, base_color_sampler, in.uv);
    return vec4<f32>(material.color.rgb, texture_color.a);
}