#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> planet_radius: f32;
@group(2) @binding(1)
var<uniform> atmosphere_radius: f32;

const COLOR = vec4<f32>(0.242, 0.617, 0.831, 1.0);

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2<f32>(0.5, 0.5);
    let dist = abs(length(uv - center) - 0.5);

    let p = planet_radius / atmosphere_radius;
    return mix(COLOR, vec4<f32>(0.0, 0.0, 0.0, 1.0), 1 - (dist * (1 - p) + p));
}
