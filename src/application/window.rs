use std::sync::Arc;

use winit::window::Window;

use crate::{graphics, map, render};

/// A window with an assosciated render state
pub struct RenderedWindow {
    /// The window, it must be in an Arc because it is shared with the render state
    pub window: Arc<Window>,
    /// The render state to render onto the window
    pub render_state: render::RenderState,
    /// The graphics state used for rendering
    pub graphics_state: graphics::State,
}

impl RenderedWindow {
    /// Constructs a new rendered window
    ///
    /// # Parameters
    ///
    /// window: The window to add a render state to
    ///
    /// graphics_settings: The settings for the graphics
    ///
    /// map: The map to render
    pub async fn new<S: map::sun::Intensity>(
        window: Window,
        graphics_settings: graphics::Settings,
        map: &map::Map<S>,
    ) -> Result<Self, render::NewRenderStateError> {
        let window = Arc::new(window);
        let render_state = render::RenderState::new(&window).await?;
        let graphics_state = graphics::State::new(&render_state, graphics_settings, map);

        return Ok(Self {
            window,
            render_state,
            graphics_state,
        });
    }
}

/// An optional rendered window with some utility
pub struct OptionalRenderedWindow(Option<RenderedWindow>);

impl OptionalRenderedWindow {
    /// Constructs a new window
    /// 
    /// # Parameters
    /// 
    /// window: The window to set
    pub fn new(window: RenderedWindow) -> Self {
        return Self(Some(window));
    }

    /// Constructs an invalid window
    pub fn empty() -> Self {
        return Self(None);
    }

    /// Retrieves a reference to the rendered window of the application
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop running the application
    pub fn get(&self) -> &RenderedWindow {
        return match &self.0 {
            Some(window) => window,
            None => panic!("Window is not initialized"),
        };
    }

    /// Retrieves a mutable reference to the rendered window of the application
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop running the application
    pub fn get_mut(&mut self) -> &mut RenderedWindow {
        return match &mut self.0 {
            Some(window) => window,
            None => panic!("Window is not initialized"),
        };
    }
}
