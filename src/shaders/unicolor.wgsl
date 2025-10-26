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
    // The fully saturated color
    saturated: vec4<f32>,
    // The color when it is the least saturated
    empty: vec4<f32>,
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
    let grid_pos = vec2<f32>((f32(column) + 0.5 * f32(row % u32(2))), -0.5 * sqrt_3 * f32(row));

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
    return color_map.saturated * in.color_value + color_map.empty * (1.0 - in.color_value);
}