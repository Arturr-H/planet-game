#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct UniformData {
    seed: f32,
}

@group(2) @binding(0)
var<uniform> u_data: UniformData;
@group(2) @binding(1)
var<uniform> radius: f32;

const V_BORDER_WIDTH: f32 = 0.1;
fn get_scale(radius: f32) -> f32 {
    return 0.3408 * radius + 18.8698;
    // return 200.0;
}
// How much the stones spread from 
// layer to layer.
const V_STONE_SPREAD: f32 = 8.0;


// const V_OFFSET: vec2<f32> = vec2<f32>(-0.002, -0.002); // Offset for the copied cells

// IMPORTANT: Don't forget to run the to_linear.py
//TOP LAYER | GRASS
const COATING_COLOR = vec3<f32>(0.107, 0.423, 0.093); // Ex. Grass color, set layer_0 to alpha 0.0 if using coating

const V_LAYER_0_SHADOW = vec4<f32>(0.00, 0.00, 0.0, 0.0); //SHOULD GENERALLY BE LIGHTER THAN FILL
const V_LAYER_0 = vec4<f32>(0.0, 0.0, 0.0, 0.0); 
const V_LAYER_0_BORDER = vec4<f32>(0.0, 0.0, 0.0, 0.0); 

const V_LAYER_1_SHADOW = vec4<f32>(0.184, 0.093, 0.070, 1.0);
const V_LAYER_1 = vec4<f32>(0.080, 0.041, 0.037, 1.0);
const V_LAYER_1_BORDER = vec4<f32>(0.032, 0.016, 0.017, 1.0);

const V_LAYER_2_SHADOW = vec4<f32>(0.080, 0.041, 0.037, 1.0); 
const V_LAYER_2 = vec4<f32>(0.032, 0.016, 0.017, 1.0); 
const V_LAYER_2_BORDER = vec4<f32>(0.012, 0.007, 0.007, 1.0); 

const V_LAYER_3_SHADOW = vec4<f32>(0.032, 0.016, 0.017, 1.0); 
const V_LAYER_3 = vec4<f32>(0.012, 0.007, 0.007, 1.0); 
const V_LAYER_3_BORDER = vec4<f32>(0.003, 0.003, 0.003, 1.0); 

const V_LAYER_4_SHADOW = vec4<f32>(0.012, 0.007, 0.007, 1.0); 
const V_LAYER_4 = vec4<f32>(0.003, 0.003, 0.003, 1.0); 
const V_LAYER_4_BORDER = vec4<f32>(0.002, 0.002, 0.002, 1.0); 


const V_LAYER_0_HEIGHT: f32 = 10; //GRASS
const V_LAYER_1_HEIGHT: f32 = 20;
const V_LAYER_2_HEIGHT: f32 = 15;
const V_LAYER_3_HEIGHT: f32 = 25;
const V_LAYER_4_HEIGHT: f32 = 0.05;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2<f32>(0.5, 0.5);
    let dist = abs(length(uv - center) - 0.5);
    let normal = get_normal(uv);

    let voronoi = calculate_voronoi(uv, dist);
    let offset_voronoi = calculate_voronoi(uv - (normal * vec2<f32>(-0.8 / radius)), dist);

    let border = voronoi.x;
    let cell_positions = vec2<f32>(voronoi.y, voronoi.z);
    let offset_cell_positions = vec2<f32>(offset_voronoi.y, offset_voronoi.z);

    let variation = (random2f(vec2<f32>(cell_positions), u_data.seed).x - 0.5) * V_STONE_SPREAD / radius;
    let cell_edge_dist = abs(length(cell_positions / get_scale(radius) - center) - 0.5) + variation;
    let offset_cell_edge_dist = abs(length(offset_cell_positions / get_scale(radius) - center) - 0.5) + variation;

    let border_color = calculate_voronoi_color(uv, dist);
    let stone_color = get_stone_color(cell_edge_dist);
    let offset_color = get_stone_shadow_color(offset_cell_edge_dist);
    
    let base_color = mix(stone_color, border_color, border);
    let is_same_cell = abs(cell_edge_dist - offset_cell_edge_dist) < 0.001;
    let offset_blend = step(offset_voronoi.x, voronoi.x) * (1.0 - border) * f32(is_same_cell);
    let final_ground_color = mix(base_color, offset_color, offset_blend);
    
    let noise_scale = vec2u(u32(uv.x * 5.0 * radius), u32(uv.y * 5.0 * radius));
    let noise = f32(pcg2d(noise_scale).x) / 4294967295.0;
    let coating = vec4<f32>(
        COATING_COLOR * (0.8 - noise * 0.2),
        1.0
    );
    let final_color = mix(coating, final_ground_color, final_ground_color.a);
    // let final_color = mix(coating, base_color, base_color.a);
    return final_color;
}

fn calculate_voronoi_color(uv: vec2<f32>, dist: f32) -> vec4<f32> {
    let layer_0_max = V_LAYER_0_HEIGHT;
    let layer_1_max = layer_0_max + V_LAYER_1_HEIGHT;
    let layer_2_max = layer_1_max + V_LAYER_2_HEIGHT;
    let layer_3_max = layer_2_max + V_LAYER_3_HEIGHT;
    // let layer_4_max = layer_4_max + V_LAYER_4_HEIGHT;

    if (1.0 - smoothstep(0.0, layer_0_max / radius, dist) > 0.0) {
        return V_LAYER_0_BORDER;
    }
    else if (1.0 - smoothstep(0.0, layer_1_max / radius, dist) > 0.0) {
        return V_LAYER_1_BORDER;
    }
    else if (1.0 - smoothstep(0.0, layer_2_max / radius, dist) > 0.0) {
        return V_LAYER_2_BORDER;
    } else if (1.0 - smoothstep(0.0, layer_3_max / radius, dist) > 0.0) {
        return V_LAYER_3_BORDER;
    }
    else {
        return V_LAYER_4_BORDER;
    }
}

fn get_stone_color(cell_edge_dist: f32) -> vec4<f32> {
    let layer_0_max = V_LAYER_0_HEIGHT;
    let layer_1_max = layer_0_max + V_LAYER_1_HEIGHT;
    let layer_2_max = layer_1_max + V_LAYER_2_HEIGHT;
    let layer_3_max = layer_2_max + V_LAYER_3_HEIGHT;
    
    if (1.0 - smoothstep(0.0, layer_0_max / radius, cell_edge_dist) > 0.0) {
        return V_LAYER_0_BORDER;
    } else if (1.0 - smoothstep(0.0, layer_1_max / radius, cell_edge_dist) > 0.0) {
        return V_LAYER_1_SHADOW;
    } else if (1.0 - smoothstep(0.0, layer_2_max / radius, cell_edge_dist) > 0.0) {
        return V_LAYER_2_SHADOW;
    } else if (1.0 - smoothstep(0.0, layer_3_max / radius, cell_edge_dist) > 0.0) {
        return V_LAYER_3_SHADOW;
    }
    else {
        return V_LAYER_4_BORDER;
    }
}
fn get_stone_shadow_color(offset_cell_edge_dist: f32) -> vec4<f32> {
    let layer_0_max = V_LAYER_0_HEIGHT;
    let layer_1_max = layer_0_max + V_LAYER_1_HEIGHT;
    let layer_2_max = layer_1_max + V_LAYER_2_HEIGHT;
    let layer_3_max = layer_2_max + V_LAYER_3_HEIGHT;
    
    if (1.0 - smoothstep(0.0, layer_0_max / radius, offset_cell_edge_dist) > 0.0) {
        return V_LAYER_0_BORDER;
    } else if (1.0 - smoothstep(0.0, layer_1_max / radius, offset_cell_edge_dist) > 0.0) {
        return V_LAYER_1;
    } else if (1.0 - smoothstep(0.0, layer_2_max / radius, offset_cell_edge_dist) > 0.0) {
        return V_LAYER_2;
    } else if (1.0 - smoothstep(0.0, layer_3_max / radius, offset_cell_edge_dist) > 0.0) {
        return V_LAYER_3;
    } else {
        return V_LAYER_4_BORDER;
    }
}

fn voronoi(x: vec2<f32>, dist: f32) -> vec3<f32> {
    let p = floor(x);
    let f = fract(x);
    
    var mb = vec2<i32>(0);
    var mr = vec2<f32>(0.0);
    var res = get_scale(radius);
    var cell_center = vec2<f32>(0.0);
    
    // First pass
    for(var j = -1; j <= 1; j++) {
        for(var i = -1; i <= 1; i++) {
            let b = vec2<f32>(f32(i), f32(j));
            let random_offset = random2f((p + b), u_data.seed);
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
    res = get_scale(radius);
    for(var j = -2; j <= 2; j++) {
        for(var i = -2; i <= 2; i++) {
            let b = vec2<f32>(f32(mb.x + i), f32(mb.y + j));
            let r = b + random2f((p + b), u_data.seed) - f;
            let d = dot(0.5 * (mr + r), normalize(r - mr));
            res = min(res, d);
        }
    }
    
    // let depth_spread = V_STONE_SPREAD * (1.0 + dist * 10); // Increase spread with depth
    // // let to_center = cell_center / get_scale(radius) - vec2<f32>(0.5);
    // // let radial_dist = length(to_center) * 2.0;
    // let center = vec2<f32>(0.5);
    // let cell_dist = abs(length(cell_center / get_scale(radius) - center));

    // let variation = smoothstep(0., random2f(cell_center, u_data.seed).x - 0.5, dist);

    // let varied_dist = (dist + variation);

    return vec3<f32>(res, cell_center.x, cell_center.y ); 
}

//returns the edge, cell center x, cell center y
fn calculate_voronoi(p: vec2<f32>, dist: f32) -> vec3<f32> {
    let data = voronoi(p * get_scale(radius), dist); // Scale factor
    return vec3<f32>(1.0 - smoothstep(V_BORDER_WIDTH, V_BORDER_WIDTH, data.x), data.y, data.z);
}

fn get_normal(uv: vec2<f32>) -> vec2<f32> {
    let centered_uv = uv - 0.5;
    let angle = atan2(centered_uv.y, centered_uv.x);
    return vec2<f32>(cos(angle), sin(angle));
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

fn random2f(p: vec2<f32>, seed: f32) -> vec2<f32> {
    let p_seed = p + vec2<f32>(seed, seed);
    // let p_comb = fract(p_seed * vec2<f32>(1.0, 1.0));
    let rnd = pcg2d(vec2u(u32(p_seed.x), u32(p_seed.y)));
    return vec2<f32>(
        f32(rnd.x) / 4294967295.0,
        f32(rnd.y) / 4294967295.0
    );
}

fn permute_four(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn fade_two(t: vec2<f32>) -> vec2<f32> { return t * t * t * (t * (t * 6. - 15.) + 10.); }