use crate::{constants::MATH_SQRT_3, map, render, types};
use wgpu::util::DeviceExt;

/// All settings for rendering
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    /// The color of the background
    pub color_background: types::Color,
}

/// A complete state for rendering
pub struct State {
    /// All of the settings for rendering
    settings: Settings,
    /// All pipelines used for rendering
    pipelines: Pipelines,
    /// All uniform buffers for the drawing of the background for the tiles
    uniforms_tiles_background: Uniforms,
    /// All uniform buffers for the drawing of the sun
    uniforms_sun: Uniforms,
    /// The buffers for drawing a single hexagon
    buffers_hexagon: BuffersVertices,
    /// The buffers for drawing a single rectangle
    buffers_rectangle: BuffersVertices,
    /// The buffers for instancing the tiles
    buffers_tiles_instance: BuffersInstance,
    /// The buffers for instancing the sun rays
    buffers_sun_instance: BuffersInstance,
}

impl State {
    /// Constructs a new graphics state
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// settings: The settings for this state
    ///
    /// map: The map to render
    pub fn new(render_state: &render::RenderState, settings: Settings, map: &map::Map) -> Self {
        // Create pipelines
        let pipelines = Pipelines::new(render_state);

        // Create the uniforms
        let uniforms_tiles_background = Uniforms::new(render_state);
        let uniforms_sun = Uniforms::new(render_state);

        // Create the buffers for the vertices
        let buffers_hexagon = BuffersVertices::new_hexagon(render_state);
        let buffers_rectangle = BuffersVertices::new_rectangle(render_state);

        // Create the buffers for the instancing
        let buffers_tiles_instance =
            BuffersInstance::new(render_state, &map.get_tile_background_data());
        let buffers_sun_instance = BuffersInstance::new(render_state, &map.get_sun_data());

        return Self {
            settings,
            pipelines,
            uniforms_tiles_background,
            uniforms_sun,
            buffers_hexagon,
            buffers_rectangle,
            buffers_tiles_instance,
            buffers_sun_instance,
        };
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
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.settings.color_background.get_wgpu()),
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

    /// Renders the state onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    pub fn render(
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
            * types::Transform2D::translate(&types::Point { x: 0.0, y: 0.5 });

        // Render the sun rays
        self.uniforms_sun
            .write_transform(render_state, &sun_transform);
        self.render_pass(
            render_state,
            view,
            &self.uniforms_sun,
            &self.buffers_rectangle,
            &self.buffers_sun_instance,
        );

        // Render the background of the tiles
        self.uniforms_tiles_background
            .write_transform(render_state, transform);
        self.render_pass(
            render_state,
            view,
            &self.uniforms_tiles_background,
            &self.buffers_hexagon,
            &self.buffers_tiles_instance,
        );
    }

    /// Sets the background color
    ///
    /// # Parameters
    ///
    /// color: The color of the background
    pub fn set_color_background(&mut self, color: types::Color) {
        self.settings.color_background = color;
    }

    /// Sets the color map for the background of the tiles
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color_map: The color map to set
    pub fn set_color_map_tiles_background(
        &self,
        render_state: &render::RenderState,
        color_map: &ColorMap,
    ) {
        self.uniforms_tiles_background
            .write_color_map(render_state, color_map);
    }

    /// Sets the color map for the sun
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color_map: The color map to set
    pub fn set_color_map_sun(&self, render_state: &render::RenderState, color_map: &ColorMap) {
        self.uniforms_sun.write_color_map(render_state, color_map);
    }

    /// Sets the grid layout
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// grid_layout: The grid layout to set
    pub fn set_grid_layout(
        &self,
        render_state: &render::RenderState,
        grid_layout: &map::GridLayout,
    ) {
        self.uniforms_tiles_background
            .write_grid_layout(render_state, grid_layout);
        self.uniforms_sun
            .write_grid_layout(render_state, grid_layout);
    }

    /// Updates the map data
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// map: The map to use for the update
    pub fn update_map(&self, render_state: &render::RenderState, map: &map::Map) {
        self.buffers_tiles_instance
            .update(render_state, &map.get_tile_background_data());
        self.buffers_tiles_instance
            .update(render_state, &map.get_sun_data());
    }

    /// Renders A single set of buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// uniforms: The uniforms to use for rendering
    ///
    /// buffers_vertices: The buffers for the vertices of the primitives to render
    ///
    /// buffers_instances: The buffers for all the instances to render
    fn render_pass(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        uniforms: &Uniforms,
        buffers_vertices: &BuffersVertices,
        buffers_instances: &BuffersInstance,
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
            self.pipelines.set(&mut render_pass);

            // Set the main uniforms
            uniforms.set(&mut render_pass);

            // Set vertices for a single hexagon
            let index_count = buffers_vertices.set(&mut render_pass);

            // Set the tile instances
            let instance_count = buffers_instances.set(&mut render_pass);

            // Draw
            render_pass.draw_indexed(0..index_count, 0, 0..instance_count);
        }

        // Submit
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
    }
}

/// Holds all render pipelines
struct Pipelines {
    /// The render pipeline for the grid fill
    fill: wgpu::RenderPipeline,
}

impl Pipelines {
    /// Constructs a new set of render pipelines
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(render_state: &render::RenderState) -> Self {
        // Create the shader
        let shader = wgpu::include_wgsl!("shader.wgsl");
        let shader = render_state.get_device().create_shader_module(shader);

        // Create the pipeline layout
        let layout =
            render_state
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline Layout Descriptor"),
                    bind_group_layouts: &[&Uniforms::bind_group_layout(render_state)],
                    push_constant_ranges: &[],
                });

        // Create the fill pipeline
        let fill =
            render_state
                .get_device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline: Fill"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[Vertex::desc(), map::InstanceTile::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: render_state.get_config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        return Self { fill };
    }

    /// Sets the pipeline
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.fill);
    }
}

/// Holds all of the global uniforms for the shader and the bind group for them
struct Uniforms {
    /// The buffer for the world to screen coordinates transform
    transform: wgpu::Buffer,
    /// The buffer for the color map data
    color_map: wgpu::Buffer,
    /// The buffer for the grid layout data
    grid_layout: wgpu::Buffer,
    /// The bind group for all uniforms
    bind_group: wgpu::BindGroup,
}

impl Uniforms {
    /// Creates a new set of uniforms for the gpu
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(render_state: &render::RenderState) -> Self {
        // Create transform buffer
        let transform = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Transform Buffer"),
                size: std::mem::size_of::<types::UniformTransform2D>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create color map buffer
        let color_map = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("ColorMap Buffer"),
                size: std::mem::size_of::<UniformColorMap>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create grid layout buffer
        let grid_layout = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("GridLayout Buffer"),
                size: std::mem::size_of::<map::UniformGridLayout>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create bind group for the uniforms
        let bind_group = render_state
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bind Group Uniforms"),
                layout: &Self::bind_group_layout(render_state),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: transform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: grid_layout.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: color_map.as_entire_binding(),
                    },
                ],
            });

        return Self {
            transform,
            color_map,
            grid_layout,
            bind_group,
        };
    }

    /// Update the transform, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// transform: The transform to apply to all vertices going from world coordinates to screen coordinates
    fn write_transform(&self, render_state: &render::RenderState, transform: &types::Transform2D) {
        render_state.get_queue().write_buffer(
            &self.transform,
            0,
            bytemuck::cast_slice(&[transform.get_data()]),
        );
    }

    /// Update the color map, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color_map: The data for the color map
    fn write_color_map(&self, render_state: &render::RenderState, color_map: &ColorMap) {
        render_state.get_queue().write_buffer(
            &self.color_map,
            0,
            bytemuck::cast_slice(&[color_map.get_data()]),
        );
    }

    /// Update the grid layout, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// grid_layout: The grid layout to write
    fn write_grid_layout(&self, render_state: &render::RenderState, grid_layout: &map::GridLayout) {
        render_state.get_queue().write_buffer(
            &self.grid_layout,
            0,
            bytemuck::cast_slice(&[grid_layout.get_data()]),
        );
    }

    /// Binds the uniforms to the given render pass
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }

    /// Creates the bind group layout for a set of uniforms
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn bind_group_layout(render_state: &render::RenderState) -> wgpu::BindGroupLayout {
        return render_state.get_device().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Uniform Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            },
        );
    }
}

/// Holds GPU buffers for all instance data
struct BuffersInstance {
    /// The data for all instances
    buffer: wgpu::Buffer,
    /// The number of instances
    count: u32,
}

impl BuffersInstance {
    /// Creates a new set of instance buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// data: The data to initialize the buffer with which also defines the length
    fn new(render_state: &render::RenderState, data: &[map::InstanceTile]) -> Self {
        // Create the instance buffer
        let buffer =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer: Tile"),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

        return Self {
            buffer,
            count: data.len() as u32,
        };
    }

    /// Updates the buffer
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// data: The data to set
    fn update(&self, render_state: &render::RenderState, data: &[map::InstanceTile]) {
        render_state
            .get_queue()
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }

    /// Sets the tile instance information for the given render pass
    ///
    /// Returns the number of indices set
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to set the vertex info for
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) -> u32 {
        // Set the vertex buffer
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));

        return self.count;
    }
}

/// Holds GPU buffers for the vertex data to draw a single tile
struct BuffersVertices {
    /// The buffer holding all four vertices of the tile
    vertices: wgpu::Buffer,
    /// The indices describing the triangles of the fill
    indices_bulk: wgpu::Buffer,
    /// The number of bulk indices
    count: u32,
}

impl BuffersVertices {
    /// Creates a set of hexagon buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new_hexagon(render_state: &render::RenderState) -> Self {
        // Create the vertices
        let vertices =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer: Hexagon"),
                    contents: bytemuck::cast_slice(&Vertex::vertices_hexagon()),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        // Create the indices for the bulk
        let indices_bulk =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer: Hexagon Bulk"),
                    contents: bytemuck::cast_slice(&Vertex::indices_bulk_hexagon()),
                    usage: wgpu::BufferUsages::INDEX,
                });

        return Self {
            vertices,
            indices_bulk,
            count: Vertex::COUNT_INDEX_BULK_HEXAGON as u32,
        };
    }

    /// Creates a set of rectangle buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new_rectangle(render_state: &render::RenderState) -> Self {
        // Create the vertices
        let vertices =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer: Rectangle"),
                    contents: bytemuck::cast_slice(&Vertex::vertices_rectangle()),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        // Create the indices for the bulk
        let indices_bulk =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer: Rectangle Bulk"),
                    contents: bytemuck::cast_slice(&Vertex::indices_bulk_rectangle()),
                    usage: wgpu::BufferUsages::INDEX,
                });

        return Self {
            vertices,
            indices_bulk,
            count: Vertex::COUNT_INDEX_BULK_RECTANGLE as u32,
        };
    }

    /// Sets the tile vertex information for the given render pass
    ///
    /// Returns the number of indices set
    ///
    /// # Parameters
    ///
    /// render_pass: The render pass to set the vertex info for
    fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) -> u32 {
        // Set the vertex buffer
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));

        // Set the index buffer and return the number of indices
        render_pass.set_index_buffer(self.indices_bulk.slice(..), wgpu::IndexFormat::Uint16);

        return self.count;
    }
}

/// All data for a color map
#[derive(Copy, Clone, Debug)]
pub struct ColorMap {
    /// The fully saturated color
    pub saturated: types::Color,
    // The color when it is the least saturated
    pub empty: types::Color,
}

impl ColorMap {
    /// Constructs the shader compatible version off a color map
    pub fn get_data(&self) -> UniformColorMap {
        return UniformColorMap {
            saturated: self.saturated.get_data(),
            empty: self.empty.get_data(),
        };
    }
}

/// All data for the color map uniform
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformColorMap {
    /// The fully saturated color
    pub saturated: [f32; 4],
    // The color when it is the least saturated
    pub empty: [f32; 4],
}

/// Describes a single vertex in the gpu
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// The position in the plane
    position: [f32; 2],
}

impl Vertex {
    const COUNT_VERTEX_HEXAGON: usize = 6;
    const COUNT_VERTEX_RECTANGLE: usize = 4;
    const COUNT_INDEX_BULK_HEXAGON: usize = 12;
    const COUNT_INDEX_BULK_RECTANGLE: usize = 6;

    /// Gets the memory description of a tile vertex
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }

    /// Generates vertices for a hexagon
    fn vertices_hexagon() -> [Self; Self::COUNT_VERTEX_HEXAGON] {
        [
            Self {
                position: [0.5, (0.5 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [0.0, (1.0 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [-0.5, (0.5 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [-0.5, -(0.5 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [0.0, -(1.0 / MATH_SQRT_3) as f32],
            },
            Self {
                position: [0.5, -(0.5 / MATH_SQRT_3) as f32],
            },
        ]
    }

    /// Generates the vertices for a rectangle
    fn vertices_rectangle() -> [Self; Self::COUNT_VERTEX_RECTANGLE] {
        [
            Self {
                position: [0.5, 0.5],
            },
            Self {
                position: [-0.5, 0.5],
            },
            Self {
                position: [-0.5, -0.5],
            },
            Self {
                position: [0.5, -0.5],
            },
        ]
    }

    /// Generates indices for the vertices for the bulk of a hexagon
    const fn indices_bulk_hexagon() -> [u16; Self::COUNT_INDEX_BULK_HEXAGON] {
        [0, 1, 2, 2, 3, 0, 3, 5, 0, 3, 4, 5]
    }

    /// Generates indices for the vertices for the bulk of a rectangle
    const fn indices_bulk_rectangle() -> [u16; Self::COUNT_INDEX_BULK_RECTANGLE] {
        [0, 1, 2, 2, 3, 0]
    }
}
