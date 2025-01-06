#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> color1: vec4<f32>;
@group(2) @binding(1) var<uniform> color2: vec4<f32>;


const wireframe_color: vec4<f32> = vec4<f32>(0.0, 1.0, 0.0, 1.0);
const wireframe_thickness: f32 = 0.001;

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
    
    // let distance_to_ekdge = min(min(uv.x, 1.0 - uv.x), min(uv.y, 1.0 - uv.y));
    // let wireframe = 1.0 - smoothstep(0.0, wireframe_thickness, distance_to_edge);
    let green_color1 = vec4<f32>(0.31, 0.59, 0.32, 1.0); 
    let green_color2 = vec4<f32>(0.16, 0.41, 0.23, 1.0); 

    let coord = vec2u(u32(uv.x * 10.0 * 2), u32(uv.y * 120.0 * 2));
    let noise = f32(pcg2d(coord).x) / 4294967295.0;

    let dirt_noise = mix(color1, color2, noise);
    let grass_noise = mix(green_color1, green_color2, noise);

    let green_threshold = 0.9;

    // Create a sharper transition
    let t = smoothstep(green_threshold, green_threshold + 0.01, uv.y);

    let final_color = mix(dirt_noise, grass_noise, t);
    return final_color;
}


// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     let uv = in.uv;



//     return mix(vec4<f32>(0.,0.,0.,0.), wireframe_color, wireframe);
// }