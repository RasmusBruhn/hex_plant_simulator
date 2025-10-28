use crate::constants::MATH_SQRT_3;

mod settings;
pub use settings::Settings;

mod state;
pub use state::State;

mod pipeline;
use pipeline::{PipelineType, Pipeline};

mod primitive;
use primitive::{BufferVertices, PrimitiveType};

mod instance;
use instance::{BufferInstance, UniformsInstance};
pub use instance::{InstanceMode, InstanceType};

/// Describes a single vertex in the gpu
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// The position in the plane
    position: [f32; 2],
}

impl Vertex {
    const _COUNT_VERTEX_HEXAGON: usize = 6;
    const _COUNT_VERTEX_RECTANGLE: usize = 4;
    const _COUNT_INDEX_BULK_HEXAGON: usize = 12;
    const _COUNT_INDEX_BULK_RECTANGLE: usize = 6;

    /// Gets the memory description of a tile vertex
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }

    /// Generates vertices for a hexagon
    const fn vertices_hexagon() -> &'static [Self] {
        return &[
            Self {
                position: [0.5, (0.5 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [0.0, (1.0 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [-0.5, (0.5 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [-0.5, -(0.5 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [0.0, -(1.0 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [0.5, -(0.5 / MATH_SQRT_3) as f32],
            },
        ];
    }

    /// Generates the vertices for a rectangle
    const fn vertices_rectangle() -> &'static [Self] {
        return &[
            Self {
                position: [0.5, 0.5],
            },
            Self {
                position: [-0.5, 0.5],
            },
            Self {
                position: [-0.5, -0.5],
            },
            Self {
                position: [0.5, -0.5],
            },
        ];
    }

    /// Generates indices for the vertices for the bulk of a hexagon
    const fn indices_bulk_hexagon() -> &'static [u16] {
        return &[0, 1, 2, 2, 3, 0, 3, 5, 0, 3, 4, 5];
    }

    /// Generates indices for the vertices for the bulk of a rectangle
    const fn indices_bulk_rectangle() -> &'static [u16] {
        return &[0, 1, 2, 2, 3, 0];
    }
}
