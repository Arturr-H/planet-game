#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> dimensions: vec2<f32>;
@group(2) @binding(1)
var<uniform> exceeded_length: u32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let adjusted_uv = vec2<f32>(uv.x, uv.y * dimensions.y);

    //the 1.04 is to ensure it is not too close to the bottom edge, 1.0 would be right on the edge
    let bent_y = clamp(
        4.0 * uv.x * uv.x - 4.0 * uv.x + 1.04,  
        0.0, 
        1.0
    );

    let thickness = 0.8 / dimensions.y;
    let distance_from_line = abs(uv.y - bent_y);

    if (distance_from_line < thickness) {
        if (exceeded_length == 1u) {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0); // no cable line
        }
        return vec4<f32>(0.008, 0.016, 0.063, 1.0); // cable line
    }
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}