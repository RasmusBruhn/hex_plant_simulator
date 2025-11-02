use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use crate::map;

use super::{ChangeMode, MainLoop};

mod resize;

mod keyboard;

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Handles a window event for the main window
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// event: The event to be handled
    pub(super) fn main_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        // Find the correct event
        match event {
            WindowEvent::CloseRequested => self.main_window_close_request(event_loop),
            WindowEvent::RedrawRequested => self.main_window_redraw_requested(),
            WindowEvent::Resized(size) => self.main_window_resized(size),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => self.main_window_keyboard_input(event_loop, device_id, event, is_synthetic),
            _ => (),
        }
    }
}
