use std::mem;

use crate::types;

/// A single tile for the map
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// The value to display
    value: f64,
}

impl Tile {
    /// Constructs a new tile
    ///
    /// # Parameters
    ///
    /// value: The value of the tile
    pub fn new(value: f64) -> Self {
        return Self { value };
    }

    /// Constructs a new empty tile
    pub fn new_empty() -> Self {
        return Self { value: 0.0 };
    }

    /// Converts the tile to shader compatible data
    pub fn get_data(&self) -> InstanceTile {
        return InstanceTile {
            color_value: self.value as f32,
        };
    }
}

/// Describes the entire map
#[derive(Clone, Debug)]
pub struct Map {
    /// All the tiles in a row first, left to right, bottom to top order
    tiles: Vec<Tile>,
    /// The intensity of the sun at each column in the range 0 to 1
    sun: Vec<f64>,
    /// The size of the grid
    size: types::ISize,
}

impl Map {
    /// Constructs a new empty map
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    pub fn new_empty(size: types::ISize) -> Self {
        let tiles = (0..size.w * size.h).map(|_| Tile::new_empty()).collect();
        let sun = (0..size.w).map(|_| 0.0).collect();

        return Self { tiles, sun, size };
    }

    /// Constructs a new map with a gradient in the y-direction
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    pub fn new_gradient_y(size: types::ISize) -> Self {
        let tiles = (0..size.w * size.h)
            .map(|i| Tile::new(((i / size.w) as f64) / ((size.h - 1) as f64)))
            .collect();
        let sun = (0..size.w).map(|_| 0.0).collect();

        return Self { tiles, sun, size };
    }

    /// Constructs a new map with a gradient in the x-direction
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    pub fn new_gradient_x(size: types::ISize) -> Self {
        let tiles = (0..size.w * size.h)
            .map(|i| Tile::new(((i % size.w) as f64) / ((size.w - 1) as f64)))
            .collect();
        let sun = (0..size.w)
            .map(|i| ((i % size.w) as f64) / ((size.w - 1) as f64))
            .collect();

        return Self { tiles, sun, size };
    }

    /// Retrieves the grid layout of the map
    pub fn get_grid_layout(&self) -> GridLayout {
        return GridLayout {
            n_columns: self.size.w,
        };
    }

    /// Retrieves the size of the map
    pub fn get_size(&self) -> &types::ISize {
        return &self.size;
    }

    /// Converts all the tiles to shader compatible data
    pub fn get_tile_background_data(&self) -> Vec<InstanceTile> {
        return self.tiles.iter().map(|tile| tile.get_data()).collect();
    }

    /// Converts all sun values to shader compatible data
    pub fn get_sun_data(&self) -> Vec<InstanceTile> {
        return self
            .sun
            .iter()
            .map(|value| InstanceTile {
                color_value: *value as f32,
            })
            .collect();
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

/// All data for the layout of the grid
#[derive(Copy, Clone, Debug)]
pub struct GridLayout {
    /// The number of columns in the grid
    pub n_columns: usize,
}

impl GridLayout {
    /// Constructs the shader compatible version off a grid layout
    pub fn get_data(&self) -> UniformGridLayout {
        return UniformGridLayout {
            n_columns: self.n_columns as u32,
        };
    }
}

/// All data for the layout of the grid
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformGridLayout {
    // The base color to scale
    pub n_columns: u32,
}
