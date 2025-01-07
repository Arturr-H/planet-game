#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const V_BORDER_WIDTH: f32 = 0.1;
const V_SCALE: f32 = 70.0;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let to_edge = uv - vec2<f32>(0.5);
    let dist = length(to_edge) * 2.0;
    let angle = atan2(to_edge.y, to_edge.x);

    let voronoi = get_border(uv);
    let border = voronoi.x;
    let cell_id = voronoi.y;

    let stone_color = get_stone_color(cell_id, dist / 2.0);
    let border_color = get_depth_color(uv, dist / 2.0);
    // let stone_dark = vec3<f32>(0.2, 0.2, 0.2);
    // let stone_light = vec3<f32>(0.4, 0.4, 0.4);

    // let random_val = random2f(floor(uv * V_SCALE)).x;
    // let stone_color = mix(stone_dark, stone_light, random_val);


    let final_color = mix(stone_color, border_color, border);
    // let rng = vec3<f32>(voronoi);
    let base_color = vec4<f32>(final_color, 1.0);
    // let grass_edge = 0.9 + sin(angle * 30.0) * 0.03;
    
    // return mix(base_color, vec4<f32>(0.31, 0.59, 0.32, 1.0), step(grass_edge, dist));
    return base_color;
}

fn get_depth_color(uv: vec2<f32>, center_dist: f32) -> vec3<f32> {
    let depth_shallow = vec3<f32>(0.212, 0.121, 0.141);
    let depth_medium = vec3<f32>(0.117, 0.071, 0.09);
    let depth_deep = vec3<f32>(0.041, 0.039, 0.039);

    if (center_dist < 0.4) {
        return depth_deep;
    } else if (center_dist < 0.45) {
        return depth_medium;
    } else {
        return depth_shallow;
    }
}

fn get_stone_color(cell_id: f32, depth: f32) -> vec3<f32> {
    // let depth_shallow = vec3<f32>(0.314, 0.223, 0.207);
    // let depth_medium = vec3<f32>(0.212, 0.121, 0.141);
    // let depth_deep = vec3<f32>(0.117, 0.071, 0.09);
    let depth_shallow = vec3<f32>(0.814, 0.223, 0.207); //röd shallow
    let depth_medium = vec3<f32>(0.212, 0.921, 0.141); // grön medium
    let depth_deep = vec3<f32>(0.117, 0.071, 0.99); // blå deep

    // let threshold = round(depth * 20.0) / 20.0;
    // let threshold = depth + cell_id * 0.1;
    // let threshold = fract(cell_id) + depth;
    // let threshold = (cell_id + depth) % 1.0;
    let threshold = floor((fract(cell_id) * 3.0));
    // let threshold = depth;
    var stone_color = vec3<f32>(0.0);

    if (threshold == 2.0) {
        stone_color = depth_shallow;
    } else if (threshold == 1.00) {
        stone_color = depth_medium;
    } else {
        stone_color = depth_deep;
    }

    return stone_color;
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
    var cell_id = 0.0;
    
    // First pass
    for(var j = -1; j <= 1; j++) {
        for(var i = -1; i <= 1; i++) {
            let b = vec2<f32>(f32(i), f32(j));
            let r = b + random2f(p + b) - f;
            let d = dot(r, r);
            
            if(d < res) {
                res = d;
                mr = r;
                mb = vec2<i32>(i, j);
                cell_id = random2f(p + b).x;
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
    
    return vec2<f32>(res, cell_id);
}

fn get_border(p: vec2<f32>) -> vec2<f32> {
    let data = voronoi_distance(p * V_SCALE); // Scale factor
    return vec2<f32>(1.0 - smoothstep(V_BORDER_WIDTH, V_BORDER_WIDTH, data.x), data.y);
}