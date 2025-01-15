// #import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::{mesh2d_view_bindings::globals}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(vertex_index & 1u) * 2.0 - 1.0;
    let y = f32((vertex_index >> 1u) & 1u) * 2.0 - 1.0;
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) * 0.5, (y + 1.0) * 0.5);
    return out;
}


fn rand(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = 0.0;

    if (rand(in.uv / 40) > 0.995) {
        let r = rand(in.uv);
        color = r * (0.85 * sin(globals.time * (r * 5.0) + 720.0 * r) + 0.95);
    }
    
    return vec4<f32>(vec3<f32>(color), 1.0) + vec4<f32>(0.001, 0.0, 0.003, 1.0);
}

// if (star_value > prob) {
//         let center = size * pos + vec2<f32>(size, size) * 0.5;
//         let t = 0.9 + 0.2 * sin(globals.time * 8.0 + (star_value - prob) / (1.0 - prob) * 45.0);
//         color = 1.0 - distance(frag_coord, center) / (0.5 * size);
//         color = color * t / (abs(frag_coord.y - center.y)) * t / (abs(frag_coord.x - center.x));
//     } else 