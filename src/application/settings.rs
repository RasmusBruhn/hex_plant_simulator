use winit::dpi::PhysicalSize;

use crate::{graphics, map, types};

/// All input settings for how to open and display a window
#[derive(Clone, Debug)]
pub struct WindowSettingsInput {
    /// The name of the application
    pub name: String,
    /// The size of the application window
    pub size: PhysicalSize<u32>,
    /// The settings for rendering
    pub graphics_settings: graphics::Settings,
}

/// All settings for how to open and display a window
#[derive(Clone, Debug)]
pub struct WindowSettings {
    /// The name of the application
    pub name: String,
    /// The size of the application window
    pub size: PhysicalSize<u32>,
    /// The settings for rendering
    pub graphics_settings: graphics::Settings,
}

impl WindowSettings {
    /// Constructs a new window settings
    ///
    /// # Parameters
    ///
    /// input: The user input settings
    pub fn new(input: WindowSettingsInput) -> Self {
        return Self {
            name: input.name,
            size: input.size,
            graphics_settings: input.graphics_settings,
        };
    }
}

/// All input settings for the shader
#[derive(Clone, Debug)]
pub struct ShaderSettingsInput {}

/// All settings for the shader
#[derive(Clone, Debug)]
pub struct ShaderSettings {
    /// The layout of the grid for displaying
    pub grid_layout: map::GridLayout,
}

impl ShaderSettings {
    /// Constructs a new shader settings
    ///
    /// # Parameters
    ///
    /// input: The user input settings
    ///
    /// grid_layout: The layout of the grid for displaying
    pub fn new(_input: ShaderSettingsInput, grid_layout: map::GridLayout) -> Self {
        return Self { grid_layout };
    }
}

/// All input settings how to view the app
#[derive(Clone, Debug)]
pub struct ViewerSettingsInput {
    /// The framerate of the application
    pub framerate: f64,
    /// The number of simulation steps per second
    pub sim_rate: f64,
    /// The multiplier when speeding up or slowing down the simulation
    pub sim_rate_mod: f64,
}

/// All settings how to view the app
#[derive(Clone, Debug)]
pub struct ViewerSettings {
    /// The framerate of the application
    pub framerate: f64,
    /// The number of simulation steps per second
    pub sim_rate: f64,
    /// The multiplier when speeding up or slowing down the simulation
    pub sim_rate_mod: f64,
    /// The home view for the camera
    pub home_view: types::View,
}

impl ViewerSettings {
    /// Constructs a new viewer settings
    ///
    /// # Parametes
    ///
    /// input: The user input settings
    ///
    /// home_view: The initial home view
    pub fn new(input: ViewerSettingsInput, home_view: types::View) -> Self {
        return Self {
            framerate: input.framerate,
            sim_rate: input.sim_rate,
            sim_rate_mod: input.sim_rate_mod,
            home_view,
        };
    }
}
