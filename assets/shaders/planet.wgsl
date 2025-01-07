#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> color1: vec4<f32>;
@group(2) @binding(1) var<uniform> color2: vec4<f32>;



// const dirt1: vec4<f32> = vec4<f32>(0.20, 0.11, 0.06, 1.0);
// const dirt2: vec4<f32> = vec4<f32>(0.13, 0.08, 0.05, 1.0);
const dirt1: vec4<f32> = vec4<f32>(0.149, 0.078, 0.05, 1.0);
const dirt2: vec4<f32> = vec4<f32>(0.12, 0.06, 0.039, 1.0);

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

fn dist(p0: vec2<f32>, pf: vec2<f32>) -> f32 {
    return sqrt((pf.x - p0.x) * (pf.x - p0.x) + (pf.y - p0.y) * (pf.y - p0.y));
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Create a checkerboard pattern
    let checkerSize: f32 = 0.1;
    let checker = floor(uv.x / checkerSize) + floor(uv.y / checkerSize);
    let isEven = (checker % 2.0) == 0.0;
    
    // Alternate between color1 and color2 based on the checkerboard pattern
    let finalColor = select(color2, color1, isEven);
    
    // Add UV coordinate visualization
    let uvColor = vec4<f32>(uv.x, uv.y, 0.0, 1.0);
    
    return mix(finalColor, uvColor, 0.5);
}
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     let uv = in.uv;

//     let coord = vec2u(u32(uv.x * 10.0 * 2), u32(uv.y * 10.0 * 2));
//     let noise = f32(pcg2d(coord).x) / 4294967295.0;
//     // let dirt_noise = mix(dirt1, dirt2, noise);
//     // let dirt_color = dirt1 * (0.5 - noise * 0.6);
//     let dirt_color = vec4<f32>(
//         dirt2.rgb * (0.6 - noise * 0.2),
//         1.0
//     );
//     let grass_noise = mix(grass1, grass2, noise);

//     let frequency: f32 = 1.0; // Controls how many waves fit in the space
//     let amplitude: f32 = 0.01; // Controls how deep the waves go

//     let sine_variation = amplitude * sin(frequency * uv.x); // 'time' can be passed as a uniform if needed

//     //The grass stops after the threshold
//     let grass_threshold = 0.9 + sine_variation;
//     let is_above_threshold = step(grass_threshold, uv.y);

//     //Color the grass darker if it is closer to the threshold
//     // let grass_color = mix(vec4<f32>(0.10, 0.30, 0.15, 1.0), grass_noise, smoothstep(grass_threshold - 0.05, grass_threshold, uv.y));
//     let grass_color = mix(grass_noise, vec4<f32>(0.0, 0.0, 0.0, 1.0), smoothstep(grass_threshold + 0.09, grass_threshold - 0.04, uv.y));

//     let jagged_noise = f32(pcg2d(vec2u(u32(uv.x * 500.0), 0u)).x) / 4294967295.0;
//     let jagged_height = 0.01; 
//     let jagged_density = 50.0;
//     let jagged_edge = step(1.0 - jagged_height * (0.5 + 0.5 * sin(uv.x * jagged_density)), uv.y + jagged_noise * jagged_height);

//     let final_grass = mix(grass_color, vec4<f32>(0.0, 0.0, 0.0, 0.0), jagged_edge);
//     let mixed_grass = mix(dirt_color, final_grass, is_above_threshold);

//     return mixed_grass;
// }