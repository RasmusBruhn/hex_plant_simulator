use crate::{render, types};

use super::{InstanceMode, InstanceType, State};

impl State {
    /// Renders an instance onto the screen
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    ///
    /// instance: The instance to render
    pub fn render(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        transform: &types::Transform2D,
        instance: &InstanceType,
    ) {
        match instance {
            InstanceType::Sun => self.render_sun(render_state, view, transform),
            InstanceType::GridBackground => self.render_background(render_state, view, transform),
        };
    }

    /// Renders the sun onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    fn render_sun(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        transform: &types::Transform2D,
    ) {
        // Get the transform for the sun rectangles
        let sun_scaling = (1.0 - transform.center.y) / transform.get_scaling_y();
        let sun_transform = transform
            * types::Transform2D::scale(&types::Point {
                x: 1.0,
                y: sun_scaling,
            })
            * types::Transform2D::translate(&types::Point { x: 0.5, y: 0.5 });

        // Render the sun rays
        let instance = InstanceMode::Sun;
        instance
            .get_type()
            .write_transform(&self.instances, render_state, &sun_transform);
        self.render_instance(render_state, view, &instance);
    }

    /// Renders the background onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    fn render_background(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        transform: &types::Transform2D,
    ) {
        let instance = InstanceMode::GridBackground(self.settings.mode_background);
        instance
            .get_type()
            .write_transform(&self.instances, render_state, transform);
        self.render_instance(render_state, view, &instance);
    }

    /// Renders A single set of buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// instance: The instance to render
    fn render_instance(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        instance: &InstanceMode,
    ) {
        // Create the encoder
        let mut encoder =
            render_state
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder: Fill"),
                });

        // Initialize the render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass: Fill"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set the pipeline for fill
            instance.pipeline().set(&self.pipelines, &mut render_pass);

            // Set vertices for the primitive
            let index_count = instance
                .get_type()
                .primitive()
                .set(&self.primitives, &mut render_pass);

            // Set the tile instances
            let instance_count = instance.get_type().set(&self.instances, &mut render_pass);

            // Draw
            render_pass.draw_indexed(0..index_count, 0, 0..instance_count);
        }

        // Submit
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
    }

    /// Clears the screen
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    pub fn clear(&self, render_state: &render::RenderState, view: &wgpu::TextureView) {
        // Create the encoder
        let mut encoder =
            render_state
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder: Fill"),
                });

        // Initialize the render pass
        {
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass: Fill"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.settings.color_clear.get_wgpu()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // Submit
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
    }
}
