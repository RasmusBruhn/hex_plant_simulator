/// Describes a single RGBA color
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    /// The red component
    r: f64,
    /// The green component
    g: f64,
    /// The blue component
    b: f64,
    /// The alpha component
    a: f64,
}

impl Color {
    /// Constructs a new color from RGBA values
    ///
    /// # Parameters
    ///
    /// r: The red component
    ///
    /// g: The green component
    ///
    /// b: The blue component
    ///
    /// a: The alpha component
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        return Self { r, g, b, a };
    }

    /// Retrieves the red component of the color
    pub const fn get_r(&self) -> f64 {
        return self.r;
    }

    /// Retrieves the green component of the color
    pub const fn get_g(&self) -> f64 {
        return self.g;
    }

    /// Retrieves the blue component of the color
    pub const fn get_b(&self) -> f64 {
        return self.b;
    }

    /// Retrieves the alpha component of the color
    pub const fn get_a(&self) -> f64 {
        return self.a;
    }

    /// Constructs the shader compatible version of a color
    pub const fn get_data(&self) -> [f32; 4] {
        return [self.r as f32, self.g as f32, self.b as f32, self.a as f32];
    }

    pub const fn get_wgpu(&self) -> wgpu::Color {
        return wgpu::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };
    }
}

/// All data for a color map
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorMap {
    /// The fully saturated color
    pub saturated: Color,
    // The color when it is the least saturated
    pub empty: Color,
}

impl ColorMap {
    /// Constructs the shader compatible version off a color map
    pub fn get_data(&self) -> UniformColorMap {
        return UniformColorMap {
            saturated: self.saturated.get_data(),
            empty: self.empty.get_data(),
        };
    }
}

/// All data for the color map uniform
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformColorMap {
    /// The fully saturated color
    pub saturated: [f32; 4],
    // The color when it is the least saturated
    pub empty: [f32; 4],
}
