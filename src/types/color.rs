use std::fmt::Debug;

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

/// Defines a color map which can export a list of 256 colors defining the map
pub trait ColorMap: Debug {
    /// Returns whether or not the color map is continuous and interpolation can
    /// be used
    fn get_continuous(&self) -> bool {
        return true;
    }

    /// Retrieves all the colors for the map
    fn get_colors(&self) -> [Color; 256];

    /// Retrieves all the colors for the map as shader compatible data
    fn get_data(&self) -> UniformColorMap {
        let colors = self
            .get_colors()
            .iter()
            .map(|color| color.get_data())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        return UniformColorMap {
            colors,
            continuous: self.get_continuous() as u32,
        };
    }
}

/// All data for the color map uniform
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformColorMap {
    /// The full spectrum of colors
    pub colors: [[f32; 4]; 256],
    /// If 0 then the color is snapped to the closest of the 256, else interpolation is used
    pub continuous: u32,
}

/// A color map with linear spacing in RGBA space between two colors
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorMapLinearRGBA {
    /// The color when it is the least saturated
    pub empty: Color,
    /// The fully saturated color
    pub saturated: Color,
}

impl ColorMap for ColorMapLinearRGBA {
    fn get_colors(&self) -> [Color; 256] {
        return (0..256)
            .map(|index| index as f64 / 255.0)
            .map(|saturation| Color {
                r: saturation * self.saturated.r + (1.0 - saturation) * self.empty.r,
                g: saturation * self.saturated.g + (1.0 - saturation) * self.empty.g,
                b: saturation * self.saturated.b + (1.0 - saturation) * self.empty.b,
                a: saturation * self.saturated.a + (1.0 - saturation) * self.empty.a,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }
}
