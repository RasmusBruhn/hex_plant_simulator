use std::time::Instant;

use winit::{application::ApplicationHandler, event::StartCause, event_loop::ActiveEventLoop};

use crate::map;

use super::{MainLoop, OptionalRenderedWindow};

impl<S: map::sun::Intensity> ApplicationHandler for MainLoop<S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.new_window(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Get the window
        let window = self.window.get();

        // Find the correct window and handle event correspondingly
        if window_id == window.window.id() {
            self.main_window_event(event_loop, event);
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => self.game_loop_init(event_loop),
            StartCause::ResumeTimeReached {
                requested_resume, ..
            } => self.game_loop_iteration(event_loop, requested_resume),
            StartCause::Poll => self.game_loop_iteration(event_loop, Instant::now()),
            _ => (),
        };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Close the window
        self.window = OptionalRenderedWindow::empty();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // Close the window
        self.window = OptionalRenderedWindow::empty();
    }
}
