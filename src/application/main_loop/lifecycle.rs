use std::time::{Duration, Instant};

use winit::event_loop::{ActiveEventLoop, ControlFlow};

use crate::map;

use super::MainLoop;

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Handles the initialization of the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop to handle
    pub(super) fn game_loop_init(&mut self, event_loop: &ActiveEventLoop) {
        // Start the event loop
        event_loop.set_control_flow(ControlFlow::Poll);

        // Set the size of the camera
        self.camera.resize(&self.settings_window.size);

        // Home the view
        self.home();
    }

    /// Run when the main window is to be closed
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    pub(super) fn main_window_close_request(&self, event_loop: &ActiveEventLoop) {
        // Stop the application
        event_loop.exit();
    }

    /// Handles the iteration of the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop to handle
    ///
    /// requested_resume: The time requested to resume
    pub(super) fn game_loop_iteration(
        &mut self,
        event_loop: &ActiveEventLoop,
        requested_resume: Instant,
    ) {
        // Update the time, make sure we do not get a backlog by skipping if we should wait until before now
        let now_time = Instant::now();

        let (new_time_frame, forward_frame) = get_new_time(
            &now_time,
            &self.state.next_frame_time,
            &requested_resume,
            self.settings_viewer.framerate,
        );
        let (new_time_sim, forward_sim) = if !self.state.flags.run_simulation {
            (new_time_frame, false)
        } else {
            get_new_time(
                &now_time,
                &self.state.next_sim_time,
                &requested_resume,
                self.settings_viewer.sim_rate,
            )
        };

        self.state.next_frame_time = new_time_frame;
        self.state.next_sim_time = new_time_sim;

        event_loop.set_control_flow(ControlFlow::WaitUntil(new_time_frame.min(new_time_sim)));

        // Get the window
        let window = self.window.get();

        // Handle frame iteration
        if forward_frame {
            // Update the camera
            if self.camera.update_transform() {
                window.window.request_redraw();
            }
        }

        // Update the simulation
        if (forward_frame && self.state.flags.iterate_simulation) || forward_sim {
            self.state.flags.iterate_simulation = false;
            self.state.flags.map_changed = true;
            self.state.flags.redraw_simulation = true;
            self.map.step();
        }

        // Request a redraw because of the simulation
        if forward_frame && self.state.flags.redraw_simulation {
            self.state.flags.redraw_simulation = false;
            window.window.request_redraw();
        }
    }
}

/// Gets the time of the next frame and whether a new frame should be rendered
///
/// # Parameters
///
/// now_time: The current time
///
/// next_time: The next scheduled time
///
/// requested_resume: The time requested to resume for this frame
///
/// framerate: The framerate to run at
fn get_new_time(
    now_time: &Instant,
    next_time: &Instant,
    requested_resume: &Instant,
    framerate: f64,
) -> (Instant, bool) {
    return if now_time < next_time {
        (next_time.clone(), false)
    } else {
        let duration = Duration::from_micros((1e6 / framerate).floor() as u64);
        let new_time = *requested_resume + duration;
        let new_next_time = if new_time < *now_time {
            *now_time + duration
        } else {
            new_time
        };
        (new_next_time, true)
    };
}
