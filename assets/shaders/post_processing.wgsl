#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    base_pixel_size: f32,
    screen_width: f32,
    screen_height: f32,
    camera_scale: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Invert the relationship between camera_scale and pixel_size
    let effective_pixel_size = settings.base_pixel_size / settings.camera_scale;

    // Calculate the number of pixels in each dimension
    let pixels_x = settings.screen_width / effective_pixel_size;
    let pixels_y = settings.screen_height / effective_pixel_size;

    // Scale the UV coordinates to fit the pixel grid
    let scaled_uv = vec2<f32>(
        round(in.uv.x * pixels_x) / pixels_x,
        round(in.uv.y * pixels_y) / pixels_y
    );

    // Sample the texture at the scaled UV coordinates
    return textureSample(screen_texture, texture_sampler, scaled_uv);
}
