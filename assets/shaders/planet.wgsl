#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const V_BORDER_WIDTH: f32 = 0.1;
const V_SCALE: f32 = 125.0;
// How much the stones spread from 
// layer to layer.
const V_STONE_SPREAD: f32 = 0.04;

const V_OFFSET: vec2<f32> = vec2<f32>(-0.002, -0.002); // Offset for the copied cells

// IMPORTANT: Don't forget to run the to_linear.py
//TOP LAYER | GRASS
const V_LAYER_0_SHADOW = vec3<f32>(0.107, 0.420, 0.095); //SHOULD GENERALLY BE LIGHTER THAN FILL
const V_LAYER_0 = vec3<f32>(0.107, 0.420, 0.095); 
const V_LAYER_0_BORDER = vec3<f32>(0.09, 0.400, 0.085); 

const V_LAYER_1_SHADOW = vec3<f32>(0.184, 0.093, 0.070);
const V_LAYER_1 = vec3<f32>(0.080, 0.041, 0.037);
const V_LAYER_1_BORDER = vec3<f32>(0.032, 0.016, 0.017);

const V_LAYER_2_SHADOW = vec3<f32>(0.080, 0.041, 0.037); 
const V_LAYER_2 = vec3<f32>(0.032, 0.016, 0.017); 
const V_LAYER_2_BORDER = vec3<f32>(0.012, 0.007, 0.007); 

const V_LAYER_3_SHADOW = vec3<f32>(0.032, 0.016, 0.017); 
const V_LAYER_3 = vec3<f32>(0.012, 0.007, 0.007); 
const V_LAYER_3_BORDER = vec3<f32>(0.003, 0.003, 0.003); 

const V_LAYER_4_SHADOW = vec3<f32>(0.012, 0.007, 0.007); 
const V_LAYER_4 = vec3<f32>(0.003, 0.003, 0.003); 
const V_LAYER_4_BORDER = vec3<f32>(0.002, 0.002, 0.002); 


const V_LAYER_0_HEIGHT: f32 = 0.03; //GRASS
const V_LAYER_1_HEIGHT: f32 = 0.05;
const V_LAYER_2_HEIGHT: f32 = 0.05;
const V_LAYER_3_HEIGHT: f32 = 0.07;
const V_LAYER_4_HEIGHT: f32 = 0.05;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let dist = abs(length(in.uv - vec2<f32>(0.5)) - 0.5);
    let normal = get_normal(uv);

    let voronoi = get_border(uv);
    let offset_voronoi = get_border(uv - (normal * V_OFFSET));

    let border = voronoi.x;
    let cell_id = voronoi.y;
    let offset_cell_id = offset_voronoi.y;

    let stone_color = vec4<f32>(get_stone_color(cell_id), 1.0);
    let border_color = vec4<f32>(get_border_color(uv, dist), 1.0);
    let offset_color = vec4<f32>(get_stone_shadow_color(offset_cell_id), 1.0);
    
    let base_color = mix(stone_color, border_color, border);
    
    let is_same_cell = abs(cell_id - offset_cell_id) < 0.0001;
    let offset_blend = step(offset_voronoi.x, voronoi.x) * (1.0 - border) * f32(is_same_cell);
    let final_color = mix(base_color, offset_color, offset_blend);
    return final_color;
}



fn get_border_color(uv: vec2<f32>, center_dist: f32) -> vec3<f32> {
    if (center_dist < (V_LAYER_0_HEIGHT / 2.0)) {
        return V_LAYER_0_BORDER;
    } else if (center_dist < (V_LAYER_0_HEIGHT / 2.0 + V_LAYER_1_HEIGHT / 2.0)) {
        return V_LAYER_1_BORDER;
    } else if (center_dist < (V_LAYER_0_HEIGHT / 2.0 + V_LAYER_1_HEIGHT / 2.0 + V_LAYER_2_HEIGHT / 2.0)) {
        return V_LAYER_2_BORDER;
    } else if (center_dist < (V_LAYER_0_HEIGHT / 2.0 + V_LAYER_1_HEIGHT / 2.0 + V_LAYER_2_HEIGHT / 2.0 + V_LAYER_3_HEIGHT / 2.0)) {
        return V_LAYER_3_BORDER;
    } else {
        return V_LAYER_4_BORDER;
    }
}

fn get_stone_color(cell_distance: f32) -> vec3<f32> {
    if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT)) {
        return V_LAYER_0_SHADOW;
    } else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT)) {
        return V_LAYER_1_SHADOW;
    }else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT - V_LAYER_2_HEIGHT)) {
        return V_LAYER_2_SHADOW;
    } else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT - V_LAYER_2_HEIGHT - V_LAYER_3_HEIGHT)) {
        return V_LAYER_3_SHADOW;
    } else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT - V_LAYER_2_HEIGHT - V_LAYER_3_HEIGHT - V_LAYER_4_HEIGHT)) {
        return V_LAYER_4_SHADOW;
    } else {
        return V_LAYER_4_BORDER;
    }
}
fn get_stone_shadow_color(cell_distance: f32) -> vec3<f32> {
    if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT)) {
        return V_LAYER_0;
    } else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT)) {
        return V_LAYER_1;
    }else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT - V_LAYER_2_HEIGHT)) {
        return V_LAYER_2;
    } else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT - V_LAYER_2_HEIGHT - V_LAYER_3_HEIGHT)) {
        return V_LAYER_3;
    } else if (cell_distance >= (1.0 - V_LAYER_0_HEIGHT - V_LAYER_1_HEIGHT - V_LAYER_2_HEIGHT - V_LAYER_3_HEIGHT - V_LAYER_4_HEIGHT)) {
        return V_LAYER_4;
    } else {
        return V_LAYER_4_BORDER;
    }
}

fn voronoi(x: vec2<f32>) -> vec2<f32> {
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

fn get_border(p: vec2<f32>) -> vec2<f32> {
    let data = voronoi(p * V_SCALE); // Scale factor
    return vec2<f32>(1.0 - smoothstep(V_BORDER_WIDTH, V_BORDER_WIDTH, data.x), data.y);
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

fn random2f(p: vec2<f32>) -> vec2<f32> {
    let rnd = pcg2d(vec2u(u32(p.x), u32(p.y)));
    return vec2<f32>(
        f32(rnd.x) / 4294967295.0,
        f32(rnd.y) / 4294967295.0
    );
}
