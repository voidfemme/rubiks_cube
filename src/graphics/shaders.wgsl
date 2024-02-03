// Vertex shader
@vertex
fn vs_main(@location(0) pos: vec4<f32>) -> @builtin(position) vec4<f32> {
    return pos;
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
