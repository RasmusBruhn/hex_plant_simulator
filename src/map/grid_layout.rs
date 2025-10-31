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
