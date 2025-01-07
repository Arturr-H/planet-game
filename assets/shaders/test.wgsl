#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    // let noise = f32(pcg2d(vec2u(u32(uv.x * 20.0), u32(uv.y * 20.0))).x) / 4294967295.0;
    
    let to_edge = uv - vec2<f32>(0.5);
    let dist = length(to_edge) * 2.0;
    let angle = atan2(to_edge.y, to_edge.x);
    
    let v = voronoi(uv, 60.0);

    //Grass
    let grass_height = 0.88;
    let grass_freq = 15.0;
    let grass_amp = 0.005;
    let edge = grass_height + sin(angle * grass_freq) * grass_amp;

    // let rgb = vec3<f32>(1.0 - noise * 1.0);
    // let rgb = vec3<f32>(v);
    // let base_color = vec4<f32>(rgb, 1.0);

    let dirt_color = vec4<f32>(
        vec4<f32>(0.12, 0.06, 0.039, 1.0).rgb * (0.6 - v * 0.3),
        1.0
    );
    return mix(dirt_color, vec4<f32>(0.31, 0.59, 0.32, 1.0), step(edge, dist));
}

fn pcg2d(p: vec2u) -> vec2u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    return v;
}

fn voronoi(uv: vec2<f32>, scale: f32) -> f32 {
    let cell_uv = floor(uv * scale);
    var closest_cell = vec2<f32>(0.0);
    var min_dist = 99999.0;
    
    for(var y = -1; y <= 1; y++) {
        for(var x = -1; x <= 1; x++) {
            let neighbor = cell_uv + vec2<f32>(f32(x), f32(y));
            let point = neighbor + vec2<f32>(
                f32(pcg2d(vec2u(u32(neighbor.x), u32(neighbor.y))).x) / 4294967295.0,
                f32(pcg2d(vec2u(u32(neighbor.x), u32(neighbor.y))).y) / 4294967295.0
            );
            let diff = point - uv * scale;
            let dist = dot(diff, diff);
            if dist < min_dist {
                min_dist = dist;
                closest_cell = neighbor;
            }
        }
    }
    return f32(pcg2d(vec2u(u32(closest_cell.x), u32(closest_cell.y))).x) / 4294967295.0;
}