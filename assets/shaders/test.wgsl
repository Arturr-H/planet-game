#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let scale = 20.0;
    let coord = vec2u(u32(uv.x * scale), u32(uv.y * scale));
    let noise = f32(pcg2d(coord).x) / 4294967295.0;
    let rgb = vec3<f32>(1.0 - noise * 1.0);
    return vec4<f32>(rgb, 1);
}

fn pcg2d(p: vec2u) -> vec2u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    return v;
}