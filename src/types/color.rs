use std::fmt::Debug;

use dyn_clone::DynClone;

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
pub trait ColorMap: DynClone + Debug {
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
        let flags = [(self.get_continuous() as u32) & 1, 0, 0, 0];

        return UniformColorMap { colors, flags };
    }
}

dyn_clone::clone_trait_object!(ColorMap);

/// All data for the color map uniform
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformColorMap {
    /// The full spectrum of colors
    pub colors: [[f32; 4]; 256],
    /// All flags for the uniform, must be this big due to sizing in wgsl
    ///
    /// 0: If set then it is continuous
    pub flags: [u32; 4],
}

/// A color map with linear spacing in RGBA space between two colors
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorMapLinearRGBA {
    /// The color when it is the least saturated
    pub empty: Color,
    /// The fully saturated color
    pub saturated: Color,
}

impl ColorMapLinearRGBA {
    /// Constructs a new linear RGBA color map
    ///
    /// # Parameters
    ///
    /// empty: The color when it is the least saturated
    ///
    /// saturated: The color when it is the most saturated
    pub const fn new(empty: Color, saturated: Color) -> Self {
        return Self { empty, saturated };
    }
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

/// A color map with discrete values, it is not continuous
#[derive(Clone, Debug, PartialEq)]
pub struct ColorMapDiscrete {
    /// All the different color map values
    pub colors: Vec<Color>,
    /// The padding color if the number of colors is less than 256
    pub unused: Color,
}

impl ColorMapDiscrete {
    /// Constructs a new discrete color map
    ///
    /// # Parameters
    ///
    /// colors: The colors of the color map, if the length is larger than 256
    /// then it is truncated and if it is shorter then it is padded
    ///
    /// unused: The color to use for padding
    pub const fn new(colors: Vec<Color>, unused: Color) -> Self {
        return Self { colors, unused };
    }
}

impl ColorMap for ColorMapDiscrete {
    fn get_continuous(&self) -> bool {
        return false;
    }

    fn get_colors(&self) -> [Color; 256] {
        let trunc_length = self.colors.len().min(256);
        return self.colors[0..trunc_length]
            .iter()
            .cloned()
            .chain((0..(256 - trunc_length)).map(|_| self.unused))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }
}
