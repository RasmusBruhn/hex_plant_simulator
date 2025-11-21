use std::mem;

use super::{DataModeBackground, settings::Settings, sun};

mod neighbor;
pub(super) use neighbor::{Neighbor, NeighborDirection, TileNeighbors, TilePos};

mod simulation;
use simulation::plant;

/// A single tile for the map
#[derive(Clone, Debug)]
pub struct Tile {
    /// The plant at this tile
    plant: plant::State,
    /// All tile state data which is not related to the plant
    data: TileData,
}

impl Tile {
    /// Constructs a new empty tile
    pub fn new() -> Self {
        let data = TileData::new();

        return Self {
            plant: plant::State::Nothing,
            data,
        };
    }

    /// Converts the tile to shader compatible data
    ///
    /// mode: The mode to display
    pub fn get_data_background(&self, mode: &DataModeBackground) -> InstanceTile {
        let value = match mode {
            DataModeBackground::Transparency => self.data.transparency,
            DataModeBackground::Light => self.data.light,
        };

        return InstanceTile {
            color_value: value as f32,
        };
    }
}

/// All state data for the tile (no plant data)
#[derive(Clone, Debug)]
struct TileData {
    /// The light transparency of this tile
    transparency: f64,
    /// The light level of this tile
    light: f64,
}

impl TileData {
    /// Constructs a new empty tile
    pub fn new() -> Self {
        return Self {
            transparency: 1.0,
            light: 0.0,
        };
    }
}

/// All data for instancing a tile
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceTile {
    /// The value to draw at this tile
    pub color_value: f32,
}

impl InstanceTile {
    /// Creates the vertex buffer description for the tile instance
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceTile>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32,
            }],
        };
    }
}
