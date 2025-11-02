use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::{graphics, map};

use super::{MainLoop, OptionalRenderedWindow, RenderedWindow};

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Constructs a new window and all associated resources for the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop running the application
    pub(super) fn new_window(&mut self, event_loop: &ActiveEventLoop) {
        // Open a new window
        let window_attributes = Window::default_attributes()
            .with_title(&self.settings_window.name)
            .with_inner_size(self.settings_window.size);

        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(error) => {
                eprintln!("Unable to create window: {:?}", error);
                event_loop.exit();
                return;
            }
        };

        // Add a render state
        self.window = match pollster::block_on(RenderedWindow::new(
            window,
            self.settings_window.graphics_settings.clone(),
            &mut self.map,
        )) {
            Ok(value) => OptionalRenderedWindow::new(value),
            Err(error) => {
                eprintln!("Unable to add render state: {:?}", error);
                event_loop.exit();
                return;
            }
        };

        // Set the grid layout and reload the graphics settings
        let window = self.window.get_mut();

        window.graphics_state.set_settings(
            &window.render_state,
            self.settings_window.graphics_settings.clone(),
        );
        window
            .graphics_state
            .set_grid_layout(&window.render_state, &self.settings_shader.grid_layout);
    }

    /// Sets the graphics settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub(super) fn set_graphics_settings(&mut self, settings: graphics::Settings) {
        // Get the window
        let window = self.window.get_mut();

        // Set the settings
        self.settings_window.graphics_settings = settings;
        window.graphics_state.set_settings(
            &window.render_state,
            self.settings_window.graphics_settings.clone(),
        );
        window.window.request_redraw();
    }

    /// Changes the display mode for the background
    ///
    /// # Parameters
    ///
    /// mode: The way to change the display mode
    pub(super) fn change_mode_background(&mut self, mode: &ChangeMode) {
        // Set the display mode
        let old_graphics_settings = &self.settings_window.graphics_settings;
        let graphics_settings = old_graphics_settings
            .clone()
            .with_mode_background(match mode {
                ChangeMode::Next => old_graphics_settings.mode_background.next(),
                ChangeMode::Prev => old_graphics_settings.mode_background.prev(),
                ChangeMode::Id(id) => map::DataModeBackground::from_id(*id),
            });
        self.set_graphics_settings(graphics_settings);

        // Update the map
        let window = self.window.get_mut();

        window
            .graphics_state
            .update_map(&window.render_state, &self.map);

        self.request_redraw();
    }
}

/// Describes how to change the display mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeMode {
    /// Change to the next mode
    Next,
    /// Change to the previous mode
    Prev,
    /// Change to a specific mode
    Id(usize),
}
