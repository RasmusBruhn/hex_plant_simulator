use crate::{map, types};

use super::MainLoop;

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Homes the view
    pub(super) fn home(&mut self) {
        // Get the height of the window relative to the width
        let height = if self.settings_window.size.width == 0 {
            1.0
        } else {
            self.settings_window.size.height as f64 / self.settings_window.size.width as f64
        };

        // Get the scales in x and y such that the view is exactly on the screen
        let scale_x = if self.settings_viewer.home_view.get_size().get_w() == 0.0 {
            0.0
        } else {
            1.0 / self.settings_viewer.home_view.get_size().get_w()
        };
        let scale_y = if self.settings_viewer.home_view.get_size().get_h() == 0.0 {
            0.0
        } else {
            height / self.settings_viewer.home_view.get_size().get_h()
        };

        // Find the scale such that both x and y is in the screen
        let scale = 2.0 * scale_x.min(scale_y);

        // Create the transform
        let transform = types::Transform2D::scale(&types::Point::new(scale, scale))
            * types::Transform2D::translate(&(-self.settings_viewer.home_view.get_center()));

        // Reset the camera and set the new transform
        self.camera.reset_keys();
        self.camera.set_transform(transform);
    }
}
