use crate::{graphics, map, types};

use super::MainLoop;

impl<S: map::sun::Intensity> MainLoop<S> {
    /// Requests a redraw to the system
    pub(super) fn request_redraw(&self) {
        self.window.get().window.request_redraw();
    }

    /// Run when the main window must be redrawn
    pub(super) fn main_window_redraw_requested(&mut self) {
        // Get the window
        let window = self.window.get();

        // Update the map data
        if self.state.flags.map_changed {
            self.state.flags.map_changed = false;
            window
                .graphics_state
                .update_map(&window.render_state, &mut self.map);
        }

        // Get the current texture view
        let output_texture = match window.render_state.get_surface().get_current_texture() {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Unable to get texture: {:?}", error);
                return;
            }
        };
        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Get the transforms for all repeats of the map
        let transform = self.camera.get_transform();
        let transform_pos = transform
            * types::Transform2D::translate(&types::Point {
                x: self.camera.get_settings().map_width,
                y: 0.0,
            });
        let transform_neg = transform
            * types::Transform2D::translate(&types::Point {
                x: -self.camera.get_settings().map_width,
                y: 0.0,
            });

        // Clear the screen
        window.graphics_state.clear(&window.render_state, &view);

        // Render the sun
        window.graphics_state.render(
            &window.render_state,
            &view,
            &transform_neg,
            &graphics::InstanceType::Sun,
        );
        window.graphics_state.render(
            &window.render_state,
            &view,
            &transform_pos,
            &graphics::InstanceType::Sun,
        );
        window.graphics_state.render(
            &window.render_state,
            &view,
            &transform,
            &graphics::InstanceType::Sun,
        );

        // Render the background of the tiles
        window.graphics_state.render(
            &window.render_state,
            &view,
            &transform_neg,
            &graphics::InstanceType::GridBackground,
        );
        window.graphics_state.render(
            &window.render_state,
            &view,
            &transform_pos,
            &graphics::InstanceType::GridBackground,
        );
        window.graphics_state.render(
            &window.render_state,
            &view,
            &transform,
            &graphics::InstanceType::GridBackground,
        );

        // Show to screen
        output_texture.present();
    }
}
