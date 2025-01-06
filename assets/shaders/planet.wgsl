#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> color1: vec4<f32>;
@group(2) @binding(1) var<uniform> color2: vec4<f32>;



const dirt1: vec4<f32> = vec4<f32>(0.20, 0.11, 0.06, 1.0);
const dirt2: vec4<f32> = vec4<f32>(0.13, 0.08, 0.05, 1.0);

const grass1 = vec4<f32>(0.31, 0.59, 0.32, 1.0); 
const grass2 = vec4<f32>(0.16, 0.41, 0.23, 1.0); 


fn pcg2d(p: vec2u) -> vec2u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    return v;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let coord = vec2u(u32(uv.x * 10.0 * 2), u32(uv.y * 120.0 * 2));
    let noise = f32(pcg2d(coord).x) / 4294967295.0;
    let dirt_noise = mix(dirt1, dirt2, noise);
    let grass_noise = mix(grass1, grass2, noise);

    let frequency: f32 = 0.7; // Controls how many waves fit in the space
    let amplitude: f32 = 0.01; // Controls how deep the waves go

    let sine_variation = amplitude * sin(frequency * uv.x); // 'time' can be passed as a uniform if needed

    let grass_threshold = 0.88 + sine_variation;
    let is_above_threshold = step(grass_threshold, uv.y);

    let mixed_grass = mix(dirt_noise, grass_noise, is_above_threshold);

    let jagged_noise = f32(pcg2d(vec2u(u32(uv.x * 500.0), 0u)).x) / 4294967295.0;
    let jagged_height = 0.01; 
    let jagged_density = 50.0;
    let jagged_edge = step(1.0 - jagged_height * (0.5 + 0.5 * sin(uv.x * jagged_density)), uv.y + jagged_noise * jagged_height);

    let final_color = mix(mixed_grass, vec4<f32>(0.0, 0.0, 0.0, 0.0), jagged_edge);
    return final_color;
}