struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(0) @binding(0) var<uniform> view_proj: mat4x4<f32>;
@group(1) @binding(0) var<uniform> params: FlagMaterialParams;

struct FlagMaterialParams {
    color: vec4<f32>,
    time: f32,
    amplitude: f32,
    frequency: f32,
    speed: f32,
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = view_proj * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return params.color;
}
