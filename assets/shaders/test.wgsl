#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const V_BORDER_WIDTH: f32 = 0.1;
const V_SCALE: f32 = 125.0;

// How much the stones spread from 
// layer to layer.
const V_STONE_SPREAD: f32 = 0.01;

// fn distance_from_surface(pos: vec3<f32>) -> f32 {
//     let radius = length(pos.xy);
//     let surface_radius = length(pos.xy);
//     return abs(radius - surface_radius);
// }

fn normalize_point(p: vec2<f32>) -> vec2<f32> {
    let len = length(p);
    if (len == 0.0) {
        return vec2<f32>(0.0);
    }
    return p / len;
}

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     let height_ratio = in.uv.y;
//     let glow = smoothstep(0.95, 1.05, height_ratio);
//         return mix(vec4<f32>(0.0, 0.0, 0.0, 1.0), vec4<f32>(1.0,1.,1.,1.,), glow);
//     }

// @fragment
// fn fragment(
//     in: VertexOutput
// ) -> @location(0) vec4<f32> {
//     let uv = in.uv; // UV coordinates from the vertex shader

//     // Checkerboard pattern setup
//     let checker_size: f32 = 0.1; // Size of each checker square
//     let checker = floor(uv.x / checker_size) + floor(uv.y / checker_size);
//     let is_even = (checker % 2.0) == 0.0;

//     // Assign color based on checkerboard position
//     let checker_color = select(vec4<f32>(0.0, 0.0, 0.0, 1.0), vec4<f32>(1.0,1.,1.,1.,), is_even);

//     // Optionally overlay the UV gradient for visualization
//     let uv_visualization = vec4<f32>(uv.x, uv.y, 0.0, 1.0);

//     let center = vec2<f32>(0.5);
//     let radius = length(in.uv - center);
//     let dist = abs(radius - 0.5);
//     let step2 = mix(vec4<f32>(1.0,1.,1.,1.0),  vec4<f32>(0.0, 0.0, 0.0, 1.0), smoothstep(0.0, 0.2, dist));

//     // Mix the checkerboard color with the UV gradient for debugging
//     let final_color = mix(checker_color, uv_visualization, 0.2);
//     let final_final = mix(final_color, step2, 0.8);


//     return final_final;
// }

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
// }

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    // let pos = in.world_position.xy;
    // let dist = distance_from_surface(pos);
    // let dist = length(in.position.xy) * 0.001;
    // let angle = atan2(to_edge.y, to_edge.x);

    let center = vec2<f32>(0.5);
    let radius = length(in.uv - center);
    let dist = abs(radius - 0.5);

    let voronoi = get_border(uv, dist);
    let border = voronoi.x;
    let cell_id = voronoi.y;

    let stone_color = get_stone_color(cell_id);
    let border_color = vec4<f32>(get_border_color(uv, dist), 1.0);

    let gradient = smoothstep(0.0, 1.0, dist);
    

    let final_color = mix(stone_color, border_color, border);
    // let base_color = vec4<f32>(final_color, 1.0);
    // let grass_edge = 0.9 + sin(angle * 30.0) * 0.03;
    
    // return mix(base_color, vec4<f32>(0.31, 0.59, 0.32, 1.0), step(grass_edge, dist));
    return final_color;
    // return mix(vec4<f32>(0.0, 0.0, 0.0, 1.0), vec4<f32>(1.0,1.,1.,1.,), in.color.x * 0.002);
}

// fn distance_from_surface(pos: vec2<f32>) -> f32 {
//     // Calculate distance from the point to origin (center)
//     let dist_from_center = length(pos);
    
//     // Get the angle of the current point
//     let angle = atan2(pos.y, pos.x);
    
//     // Get normalized distance for gradient
//     let normalized_dist = smoothstep(0.0, 0.3, abs(dist_from_center));
//     return normalized_dist;
// }

fn get_border_color(uv: vec2<f32>, center_dist: f32) -> vec3<f32> {
    let depth_shallow = vec3<f32>(0.212, 0.121, 0.141);
    let depth_medium = vec3<f32>(0.117, 0.071, 0.09);
    let depth_deep = vec3<f32>(0.041, 0.039, 0.039);
    let depth_grass = vec3<f32>(0.36, 0.61, 0.34);

    if (center_dist < 0.025) {
        return depth_grass;
    } else if (center_dist < 0.05) {
        return depth_shallow;
    } else if (center_dist < 0.075) {
        return depth_medium;
    } else {
        return depth_deep;
    }
}

fn get_stone_color(cell_distance: f32) -> vec4<f32> {
    let depth_grass = vec4<f32>(0.36, 0.68, 0.34, 1.0);
    let depth_shallow = vec4<f32>(0.314, 0.223, 0.207, 1.0);
    let depth_medium = vec4<f32>(0.212, 0.121, 0.141, 1.0);
    let depth_deep = vec4<f32>(0.117, 0.071, 0.09, 1.0);
    let depth_deepest = vec4<f32>(0.041, 0.039, 0.039, 1.0);
    
    // var stone_color = vec3<f32>(0.0);

    if (cell_distance >= 0.95) {
        return depth_grass;
    } else if (cell_distance >= 0.9) {
        return depth_shallow;
    }
     else if (cell_distance >= 0.85) {
        return depth_medium;
    } else if (cell_distance >= 0.8) {
        return depth_deep;
    } else {
        return depth_deepest;
    }

    // return stone_color;
}

fn pcg2d(p: vec2u) -> vec2u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    return v;
}

fn random2f(p: vec2<f32>) -> vec2<f32> {
    let rnd = pcg2d(vec2u(u32(p.x), u32(p.y)));
    return vec2<f32>(
        f32(rnd.x) / 4294967295.0,
        f32(rnd.y) / 4294967295.0
    );
}

fn voronoi_distance(x: vec2<f32>) -> vec2<f32> {
    let p = floor(x);
    let f = fract(x);
    
    var mb = vec2<i32>(0);
    var mr = vec2<f32>(0.0);
    var res = V_SCALE;
    var cell_center = vec2<f32>(0.0);
    
    // First pass
    for(var j = -1; j <= 1; j++) {
        for(var i = -1; i <= 1; i++) {
            let b = vec2<f32>(f32(i), f32(j));
            let random_offset = random2f(p + b);
            let r = b + random_offset - f;
            let d = dot(r, r);
            
            if(d < res) {
                res = d;
                mr = r;
                mb = vec2<i32>(i, j);
                cell_center = p + b + random_offset;
            }
        }
    }
    
    // Second pass
    res = V_SCALE;
    for(var j = -2; j <= 2; j++) {
        for(var i = -2; i <= 2; i++) {
            let b = vec2<f32>(f32(mb.x + i), f32(mb.y + j));
            let r = b + random2f(p + b) - f;
            let d = dot(0.5 * (mr + r), normalize(r - mr));
            res = min(res, d);
        }
    }
    let to_center = cell_center / V_SCALE - vec2<f32>(0.5);
    let radial_dist = length(to_center) * 2.0;
    let variation = (random2f(cell_center).x - 0.5) * V_STONE_SPREAD;
    let varied_dist = radial_dist + variation;

    return vec2<f32>(res, varied_dist);
}

fn get_border(p: vec2<f32>, surface_y: f32) -> vec2<f32> {
    let data = voronoi_distance(p * V_SCALE); // Scale factor
    return vec2<f32>(1.0 - smoothstep(V_BORDER_WIDTH, V_BORDER_WIDTH, data.x), data.y);
}