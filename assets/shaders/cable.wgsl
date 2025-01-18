#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> aspect_ratio: f32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // Define points A and B in UV space (e.g., top-left to top-right)

    // Define a bent line using a quadratic curve (y = ax^2 + bx + c)
    let a = 4.0; // Controls curvature
    let b = -4.0;  // Linear coefficient
    let c = 1.02;  // Adjust to fit the line

    // Calculate the bent line's Y value for the current X
    let bent_y = a * (uv.x * uv.x )+ b * uv.x + c;

    // Determine the line's thickness
    let thickness = 0.05; // Adjust for desired line thickness
    let distance_from_line = abs(uv.y - bent_y);

    // If the fragment is within the line's thickness, color it
    if (distance_from_line < thickness) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Green line
    }

    // Otherwise, render background color
    return vec4<f32>(0.0, 0.0, 0.0, 0.0); // Black background
}