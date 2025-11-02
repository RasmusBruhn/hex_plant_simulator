use winit::dpi::PhysicalSize;

use crate::map;

use super::MainLoop;

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Run when the size of the window has changed
    ///
    /// # Parameters
    ///
    /// size: The new size of the window
    pub(super) fn main_window_resized(&mut self, size: PhysicalSize<u32>) {
        // Skip if it is zero
        if size.width == 0 || size.height == 0 {
            return;
        }

        // Set the new size
        self.settings_window.size = size;

        // Update the window
        self.window.get_mut().render_state.resize(size);

        // Update the camera
        self.camera.resize(&size);
    }
}
