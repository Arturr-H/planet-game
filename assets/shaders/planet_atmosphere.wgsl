#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> planet_radius: f32;
@group(2) @binding(1)
var<uniform> zoom: f32;

const COLOR = vec4<f32>(0.242, 0.617, 0.831, 1.0);

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2<f32>(0.5, 0.5);

    // let p = planet_radius / atmosphere_radius;
    let dist = abs(length(uv - center) - 0.5);
    let zoom_1 = mix(vec4<f32>(0.024, 0.100, 0.270, smoothstep(0.0, 180 / planet_radius, dist)), vec4(1.0, 1.0, 1.0, 1.0), smoothstep(0.0, 120 / planet_radius, dist));
    // let zoom_0 = vec4<f32>(0.521, 0.807, 0.913, smoothstep(0.0, 100 / planet_radius, dist));
    let zoom_0 = vec4<f32>(0.821, 0.207, 0.113, smoothstep(0.0, 100 / planet_radius, dist));

    return mix(zoom_0, zoom_1, smoothstep(0.0, 0.3, zoom / 10));
    // return zoom_0;
}
