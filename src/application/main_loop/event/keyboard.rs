use winit::{
    event::{DeviceId, ElementState, KeyEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::map;

use super::{ChangeMode, MainLoop};

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Handles any keyboard input like camera movement
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// device_id: The id of the device giving the input
    ///
    /// event: The event to handle
    ///
    /// is_synthetic: True if the event was created by winit
    pub(super) fn main_window_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
    ) {
        // Handle camera events
        _ = self.camera.apply_key(&event);

        // Handle all non-repeating pressed keys
        let mut update = false;
        if event.state == ElementState::Pressed && !event.repeat {
            update |= self.main_window_keyboard_input_pressed(event_loop, event.physical_key);
        }

        if event.state == ElementState::Released && !event.repeat {
            update |= self.main_window_keyboard_input_released(event_loop, event.physical_key);
        }

        // Handle all repeating key presses
        if event.state == ElementState::Pressed {
            update |= self.main_window_keyboard_input_repeated(event_loop, event.physical_key);
        }

        // Update the graphics
        if update {
            let window = self.window.get();
            window.window.request_redraw();
        }
    }

    /// Handles all keys pressed a single time, returns true if an update is needed
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// key: The key which has been pressed
    fn main_window_keyboard_input_pressed(
        &mut self,
        event_loop: &ActiveEventLoop,
        key: PhysicalKey,
    ) -> bool {
        let mut update = false;
        match key {
            PhysicalKey::Unidentified(_) => (),
            PhysicalKey::Code(code) => match code {
                KeyCode::Escape => {
                    // Close the application
                    event_loop.exit();
                }
                KeyCode::KeyH => {
                    // Return to home view
                    self.home();
                    update = true;
                }
                KeyCode::Space => {
                    // Toggle the simulation
                    self.state.flags.run_simulation = !self.state.flags.run_simulation;
                }
                KeyCode::Tab => {
                    // Change the speed of the simulation
                    if self.state.flags.left_shift_active {
                        self.settings_viewer.sim_rate /= self.settings_viewer.sim_rate_mod;
                    } else {
                        self.settings_viewer.sim_rate *= self.settings_viewer.sim_rate_mod;
                    }
                }
                KeyCode::ShiftLeft => {
                    // Toggle the shift key
                    self.state.flags.left_shift_active = true;
                }
                KeyCode::Digit1 => {
                    // Go to background display mode 0
                    self.change_mode_background(&ChangeMode::Id(0));
                }
                KeyCode::Digit2 => {
                    // Go to background display mode 1
                    self.change_mode_background(&ChangeMode::Id(1));
                }
                KeyCode::Digit3 => {
                    // Go to background display mode 2
                    self.change_mode_background(&ChangeMode::Id(2));
                }
                KeyCode::Digit4 => {
                    // Go to background display mode 3
                    self.change_mode_background(&ChangeMode::Id(3));
                }
                KeyCode::Digit5 => {
                    // Go to background display mode 4
                    self.change_mode_background(&ChangeMode::Id(4));
                }
                KeyCode::Digit6 => {
                    // Go to background display mode 5
                    self.change_mode_background(&ChangeMode::Id(5));
                }
                KeyCode::Digit7 => {
                    // Go to background display mode 6
                    self.change_mode_background(&ChangeMode::Id(6));
                }
                KeyCode::Digit8 => {
                    // Go to background display mode 7
                    self.change_mode_background(&ChangeMode::Id(7));
                }
                KeyCode::Digit9 => {
                    // Go to background display mode 8
                    self.change_mode_background(&ChangeMode::Id(8));
                }
                KeyCode::Digit0 => {
                    // Go to background display mode 9
                    self.change_mode_background(&ChangeMode::Id(9));
                }
                _ => (),
            },
        };

        return update;
    }

    /// Handles all keys release, returns true if an update is needed
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// key: The key which has been released
    fn main_window_keyboard_input_released(
        &mut self,
        _event_loop: &ActiveEventLoop,
        key: PhysicalKey,
    ) -> bool {
        match key {
            PhysicalKey::Unidentified(_) => (),
            PhysicalKey::Code(code) => match code {
                KeyCode::ShiftLeft => {
                    // Toggle the shift key
                    self.state.flags.left_shift_active = false;
                }
                _ => (),
            },
        };

        return false;
    }

    /// Handles all keys pressed repeatedly, returns true if an update is needed
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// key: The key which has been pressed
    fn main_window_keyboard_input_repeated(
        &mut self,
        _event_loop: &ActiveEventLoop,
        key: PhysicalKey,
    ) -> bool {
        match key {
            PhysicalKey::Unidentified(_) => (),
            PhysicalKey::Code(code) => match code {
                KeyCode::Enter => {
                    // Forward the simulation once
                    self.state.flags.iterate_simulation = true;
                }
                KeyCode::ArrowRight => {
                    // Go to the next background display mode
                    self.change_mode_background(&ChangeMode::Next);
                }
                KeyCode::ArrowLeft => {
                    // Go to the previous background display mode
                    self.change_mode_background(&ChangeMode::Prev);
                }
                _ => (),
            },
        };

        return false;
    }
}
