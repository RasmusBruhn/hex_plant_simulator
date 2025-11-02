// Structs
// The structure to input for the vertex shader
struct VertexInput {
    // The position for the vertex in world coordinates
    @location(0) pos: vec2<f32>,
}

// The instance input for the vertex shader
struct InstanceInput {
    // The index of the tile
    @builtin(instance_index) id: u32,
    // The color for the tile
    @location(1) color_value: f32,
}

// The stucture to output for the vertex shader
struct VertexOutput {
    // The position of the vertex in screen coordinates
    @builtin(position) clip_position: vec4<f32>,
    // The value to display
    @location(0) color_value: f32,
};

// A transformation in 2D
struct Transform2D {
    // The transformation matrix
    transform: mat4x4<f32>,
};

// All information to do with the color map
struct ColorMap {
    // The full list of colors for the color map
    colors: array<vec4<f32>, 256>,
    // All flags for the uniform, must be this big due to sizing in wgsl
    //
    // 0: If set then it is continuous
    flags: vec4<u32>,
}

// All information on the layout of the grid
struct GridLayout {
    // The number of columns
    n_columns: u32,
}

// Uniforms
// The transform to apply to each vertex
@group(0) @binding(0)
var<uniform> transform: Transform2D;

// The number of columns in the grid
@group(0) @binding(1)
var<uniform> grid_layout: GridLayout;

// The information for the color map
@group(0) @binding(2)
var<uniform> color_map: ColorMap;

const sqrt_3: f32 = 1.73205080756887729352744634150587236694280525381038062805580697945193301690;

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    // Get the position in the grid
    let column = instance.id % grid_layout.n_columns;
    let row = instance.id / grid_layout.n_columns;
    let grid_pos = vec2<f32>((f32(column) + 0.5 * f32(row % 2u)) * 1.05, -0.5 * sqrt_3 * f32(row));

    // Get the position on the screen
    let screen_pos = transform.transform * vec4<f32>(model.pos + grid_pos, 0.0, 1.0);

    // Create the output
    var out: VertexOutput;
    out.clip_position = screen_pos;
    out.color_value = instance.color_value;
    return out;
}

// Fragment shader
@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    // Check if the color map is continuous
    let continuous = (color_map.flags.x & 1u) != 0u;

    // Clamp the color value to avoid overflow
    let color_value = clamp(in.color_value, 0.0, 1.0) * 255.0;

    // Handle non-continuous color maps by snapping
    if (!continuous) {
        let color_index = u32(color_value + 0.5);
        return color_map.colors[color_index];
    }

    // Handle continuous color maps
    let color_index = u32(color_value);
    let color_ratio = color_value - f32(color_index);

    // Handle the max value differently
    if (color_index == 255u) {
        return color_map.colors[color_index];
    }
    return color_ratio * color_map.colors[color_index + 1u] + (1.0 - color_ratio) * color_map.colors[color_index];
}