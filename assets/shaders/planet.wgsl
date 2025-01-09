#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const V_BORDER_WIDTH: f32 = 0.1;
const V_SCALE: f32 = 125.0;
// How much the stones spread from 
// layer to layer.
const V_STONE_SPREAD: f32 = 0.02;

// IMPORTANT: Don't forget to run the to_linear.py
// Stone colors, V = Voronoi, S = Stone
const V_S_DEPTH_GRASS = vec4<f32>(0.107, 0.420, 0.095, 1.0);
const V_S_DEPTH_SHALLOW = vec4<f32>(0.314, 0.223, 0.207, 1.0);
const V_S_DEPTH_MEDIUM = vec4<f32>(0.212, 0.121, 0.141, 1.0);
const V_S_DEPTH_DEEP = vec4<f32>(0.117, 0.071, 0.09, 1.0);
const V_S_DEPTH_DEEPEST = vec4<f32>(0.041, 0.039, 0.039, 1.0);

// Border colors V = Voronoi, B = Border
const V_B_DEPTH_SHALLOW = vec3<f32>(0.212, 0.121, 0.141);
const V_B_DEPTH_MEDIUM = vec3<f32>(0.117, 0.071, 0.09);
const V_B_DEPTH_DEEP = vec3<f32>(0.041, 0.039, 0.039);
const V_B_DEPTH_GRASS = vec3<f32>(0.107, 0.390, 0.095);

fn normalize_point(p: vec2<f32>) -> vec2<f32> {
    let len = length(p);
    if (len == 0.0) {
        return vec2<f32>(0.0);
    }
    return p / len;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

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
    return final_color;
}


fn get_border_color(uv: vec2<f32>, center_dist: f32) -> vec3<f32> {
    if (center_dist < 0.025) {
        return V_B_DEPTH_GRASS;
    } else if (center_dist < 0.05) {
        return V_B_DEPTH_SHALLOW;
    } else if (center_dist < 0.075) {
        return V_B_DEPTH_MEDIUM;
    } else {
        return V_B_DEPTH_DEEP;
    }
}

fn get_stone_color(cell_distance: f32) -> vec4<f32> {
    if (cell_distance >= 0.95) {
        return V_S_DEPTH_GRASS;
    } else if (cell_distance >= 0.9) {
        return V_S_DEPTH_SHALLOW;
    }else if (cell_distance >= 0.85) {
        return V_S_DEPTH_MEDIUM;
    } else if (cell_distance >= 0.8) {
        return V_S_DEPTH_DEEP;
    } else {
        return V_S_DEPTH_DEEPEST;
    }
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

// -- Hashing functions --
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
