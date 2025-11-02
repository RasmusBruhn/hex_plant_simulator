use crate::{camera, constants::MATH_SQRT_3, map, types};

use super::{
    OptionalRenderedWindow, RenderedWindow, ShaderSettings, ShaderSettingsInput, State,
    ViewerSettings, ViewerSettingsInput, WindowSettings, WindowSettingsInput,
};

mod state;
use state::ChangeMode;

mod view;

mod redraw;

mod lifecycle;

mod event;

mod application_handler;

/// Controls the main game loop of the application
pub struct MainLoop<S: map::sun::Intensity> {
    /// The currently opened window of the application
    window: OptionalRenderedWindow,
    /// The map of tiles
    map: map::Map<S>,
    /// The camera for controlling what is displayed
    camera: camera::Camera,
    /// All the settings for creating and displaying a window
    settings_window: WindowSettings,
    /// All settings for the shader
    settings_shader: ShaderSettings,
    /// All settings for viewing the application
    settings_viewer: ViewerSettings,
    /// The state of the viewer
    state: State,
}

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Creates a new main loop with the supplied settings
    ///
    /// # Parameters
    ///
    /// map: The full map
    ///
    /// camera: The camera for controlling what is displayed
    ///
    /// settings_window: All the settings for creating and displaying a window
    ///
    /// settings_shader: All settings for the shader
    ///
    /// settings_viewer: All settings for viewing the application
    pub fn new(
        map: map::Map<S>,
        mut camera: camera::Camera,
        settings_window: WindowSettingsInput,
        settings_shader: ShaderSettingsInput,
        settings_viewer: ViewerSettingsInput,
    ) -> Self {
        // Set the width of the map in the camera
        let camera_settings = camera
            .get_settings()
            .with_map_width(map.get_size().w as f64);
        camera.set_settings(camera_settings);

        // Create the window settings
        let settings_window = WindowSettings::new(settings_window);

        // Create the shader settings
        let settings_shader = ShaderSettings::new(settings_shader, map.get_grid_layout());

        // Create the viewer settings
        let home_view = types::View::new(
            types::Point::new(
                ((map.get_size().w - 1) as f64) * 0.5,
                -((map.get_size().h - 1) as f64) / MATH_SQRT_3 * 0.5,
            ),
            types::Size::new(
                map.get_size().w as f64,
                map.get_size().h as f64 / MATH_SQRT_3,
            ),
        );
        let settings_viewer = ViewerSettings::new(settings_viewer, home_view);

        return Self {
            window: OptionalRenderedWindow::empty(),
            map,
            camera,
            settings_window,
            settings_shader,
            settings_viewer,
            state: State::new(),
        };
    }
}
