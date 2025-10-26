use crate::{constants::MATH_SQRT_3, map, render, types};
use wgpu::util::DeviceExt;

/// All settings for rendering
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    /// The color of the background
    pub color_background: types::Color,
    /// The display mode for the background of the tiles
    pub mode_tiles_background: map::DataModeBackground,
}

/// A complete state for rendering
pub struct State {
    /// All of the settings for rendering
    settings: Settings,
    /// All pipelines used for rendering
    pipelines: [Pipelines; PipelineType::COUNT],
    /// All vertex buffers
    primitives: [BuffersVertices; PrimitiveType::COUNT],
    /// All instance data both buffers and uniforms
    instances: [(BuffersInstance, UniformsInstance); InstanceType::COUNT],
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
        let pipelines = PipelineType::new_collection(render_state);

        // Create the primitives
        let primitives = PrimitiveType::new_collection(render_state);

        // Create the instance buffers and uniforms
        let instances =
            InstanceType::new_collection(render_state, map, settings.mode_tiles_background);

        return Self {
            settings,
            pipelines,
            primitives,
            instances,
        };
    }

    /// Retrieves a reference to the settings
    pub fn get_settings(&self) -> &Settings {
        return &self.settings;
    }

    /// Retrieves a mutable reference to the settings
    pub fn get_settings_mut(&mut self) -> &mut Settings {
        return &mut self.settings;
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

    /// Renders the sun onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    pub fn render_sun(
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
        let instance_type = InstanceType::Sun;
        instance_type.write_transform(&self.instances, render_state, &sun_transform);
        self.render_instance(render_state, view, &instance_type);
    }

    /// Renders the tile background onto the given view
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// view: The texture view to render to
    ///
    /// transform: The transform to go from world to screen coordinates
    pub fn render_tiles_background(
        &self,
        render_state: &render::RenderState,
        view: &wgpu::TextureView,
        transform: &types::Transform2D,
    ) {
        let instance_type = InstanceType::GridBackground(self.settings.mode_tiles_background);
        instance_type.write_transform(&self.instances, render_state, transform);
        self.render_instance(render_state, view, &instance_type);
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
    /// instance: The instance to set the color map for
    ///
    /// color_map: The color map to set
    pub fn set_color_map(
        &self,
        render_state: &render::RenderState,
        instance: &InstanceType,
        color_map: &ColorMap,
    ) {
        instance.write_color_map(&self.instances, render_state, color_map);
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
        InstanceType::write_grid_layout_collection(&self.instances, render_state, grid_layout);
    }

    /// Updates the map data
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// map: The map to use for the update
    pub fn update_map(&self, render_state: &render::RenderState, map: &map::Map) {
        InstanceType::update_collection(
            &self.instances,
            render_state,
            map,
            self.settings.mode_tiles_background,
        );
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
        instance: &InstanceType,
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
            instance.pipeline().set(&self.pipelines, &mut render_pass);

            // Set vertices for the primitive
            let index_count = instance.primitive().set(&self.primitives, &mut render_pass);

            // Set the tile instances
            let instance_count = instance.set(&self.instances, &mut render_pass);

            // Draw
            render_pass.draw_indexed(0..index_count, 0, 0..instance_count);
        }

        // Submit
        render_state
            .get_queue()
            .submit(std::iter::once(encoder.finish()));
    }
}

/// Describes which pipeline to use
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PipelineType {
    /// All object are rendered with a uniform color from a color map
    Unicolor,
}

impl PipelineType {
    /// The number of different pipelines
    pub const COUNT: usize = 1;

    /// The id to find the pipeline in the pipeline list
    fn id(&self) -> usize {
        return match self {
            Self::Unicolor => 0,
        };
    }

    /// Constructs a new pipeline matching the pipeline type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(&self, render_state: &render::RenderState) -> Pipelines {
        let shader = match self {
            Self::Unicolor => wgpu::include_wgsl!("shaders/unicolor.wgsl"),
        };

        return Pipelines::new(render_state, shader);
    }

    /// Constructs the pipelines for all the different pipeline type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new_collection(render_state: &render::RenderState) -> [Pipelines; Self::COUNT] {
        return [Self::Unicolor.new(render_state)];
    }

    /// Sets the correct pipeline from the collection
    ///
    /// # Parameters
    ///
    /// collection: The full collection of pipelines
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(
        &self,
        collection: &'a [Pipelines; Self::COUNT],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        collection[self.id()].set(render_pass);
    }
}

/// Holds all render pipelines for a single pipeline type
struct Pipelines {
    /// The render pipeline for the fill
    fill: wgpu::RenderPipeline,
}

impl Pipelines {
    /// Constructs a new set of render pipelines
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// shader: The shader descriptor
    fn new(render_state: &render::RenderState, shader: wgpu::ShaderModuleDescriptor) -> Self {
        // Create the shader
        let shader = render_state.get_device().create_shader_module(shader);

        // Create the pipeline layout
        let layout =
            render_state
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline Layout Descriptor"),
                    bind_group_layouts: &[&UniformsInstance::bind_group_layout(render_state)],
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

/// Describes which primitive (square, hexagon, ect.) to draw
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    /// Draw hexagons
    Hexagon,
    /// Draw rectangles
    Rectangle,
}

impl PrimitiveType {
    /// The number of different primitives
    pub const COUNT: usize = 2;

    /// The id to find the primitive in the buffer list
    fn id(&self) -> usize {
        return match self {
            Self::Hexagon => 0,
            Self::Rectangle => 1,
        };
    }

    /// Constructs a new pipeline matching the pipeline type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new(&self, render_state: &render::RenderState) -> BuffersVertices {
        let (vertices, bulk_indices) = match self {
            Self::Hexagon => (Vertex::vertices_hexagon(), Vertex::indices_bulk_hexagon()),
            Self::Rectangle => (
                Vertex::vertices_rectangle(),
                Vertex::indices_bulk_rectangle(),
            ),
        };

        return BuffersVertices::new(render_state, vertices, bulk_indices);
    }

    /// Constructs the primitive vertices for all the different primitive type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn new_collection(render_state: &render::RenderState) -> [BuffersVertices; Self::COUNT] {
        return [
            Self::Hexagon.new(render_state),
            Self::Rectangle.new(render_state),
        ];
    }

    /// Sets the correct primitive from the collection, returns the number of indices set
    ///
    /// # Parameters
    ///
    /// collection: The full collection of primitives
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(
        &self,
        collection: &'a [BuffersVertices; Self::COUNT],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> u32 {
        return collection[self.id()].set(render_pass);
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
    /// Creates a set of buffers
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// vertices: The list of vertices describing the primitive
    ///
    /// bulk_indices: The list of pairs of 3 indices describing all the triangles defining the primitive fill
    fn new(render_state: &render::RenderState, vertices: &[Vertex], bulk_indices: &[u16]) -> Self {
        // Create the vertices
        let vertices =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        // Create the indices for the bulk
        let indices_bulk =
            render_state
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer: Hexagon Bulk"),
                    contents: bytemuck::cast_slice(bulk_indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        return Self {
            vertices,
            indices_bulk,
            count: bulk_indices.len() as u32,
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

/// Describes which instance to use
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceType {
    /// Instances for the background of the grid
    GridBackground(map::DataModeBackground),
    /// Instances for the sun data
    Sun,
}

impl InstanceType {
    /// The number of different instance types
    pub const COUNT: usize = 2;

    /// The id to find the instance type in the instance list
    fn id(&self) -> usize {
        return match self {
            Self::GridBackground(_) => 0,
            Self::Sun => 1,
        };
    }

    /// Gets the primitive type used for this instance
    fn primitive(&self) -> PrimitiveType {
        return match self {
            Self::GridBackground(_) => PrimitiveType::Hexagon,
            Self::Sun => PrimitiveType::Rectangle,
        };
    }

    /// Gets the pipeline used for this primitive
    fn pipeline(&self) -> PipelineType {
        return match self {
            Self::GridBackground(_) | Self::Sun => PipelineType::Unicolor,
        };
    }

    /// Gets the data used for this instance
    ///
    /// # Parameters
    ///
    /// map: The map used to get data from
    fn data(&self, map: &map::Map) -> Vec<map::InstanceTile> {
        return match self {
            Self::GridBackground(mode) => map.get_tile_data_background(&mode),
            Self::Sun => map.get_sun_data(),
        };
    }

    /// Constructs a new instance buffer and uniforms matching the instance type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// map: The map to use for initialization of the data
    fn new(
        &self,
        render_state: &render::RenderState,
        map: &map::Map,
    ) -> (BuffersInstance, UniformsInstance) {
        return (
            BuffersInstance::new(render_state, &self.data(map)),
            UniformsInstance::new(render_state),
        );
    }

    /// Constructs the instance buffers and uniforms for all the different instance types
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// map: The map to use for initialization of the data
    ///
    /// mode_tiles_background: The display mode for the background of the tiles
    fn new_collection(
        render_state: &render::RenderState,
        map: &map::Map,
        mode_tiles_background: map::DataModeBackground,
    ) -> [(BuffersInstance, UniformsInstance); Self::COUNT] {
        return [
            Self::GridBackground(mode_tiles_background).new(render_state, map),
            Self::Sun.new(render_state, map),
        ];
    }

    /// Updates a instance buffer matching the instance type
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// map: The map to use for data
    fn update(
        &self,
        collection: &[(BuffersInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        map: &map::Map,
    ) {
        collection[self.id()]
            .0
            .update(render_state, &self.data(map));
    }

    /// Updates the instance buffers for all the different instance types
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// map: The map to use for data
    ///
    /// mode_tiles_background: The display mode for the background of the tiles
    fn update_collection(
        collection: &[(BuffersInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        map: &map::Map,
        mode_tiles_background: map::DataModeBackground,
    ) {
        Self::GridBackground(mode_tiles_background).update(collection, render_state, map);
        Self::Sun.update(collection, render_state, map);
    }

    /// Update the transform, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// transform: The transform to apply to all vertices going from world coordinates to screen coordinates
    fn write_transform(
        &self,
        collection: &[(BuffersInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        transform: &types::Transform2D,
    ) {
        collection[self.id()]
            .1
            .write_transform(render_state, transform);
    }

    /// Update the color map, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color_map: The data for the color map
    fn write_color_map(
        &self,
        collection: &[(BuffersInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        color_map: &ColorMap,
    ) {
        collection[self.id()]
            .1
            .write_color_map(render_state, color_map);
    }

    /// Update the grid layout, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// grid_layout: The grid layout to write
    fn write_grid_layout(
        &self,
        collection: &[(BuffersInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        grid_layout: &map::GridLayout,
    ) {
        collection[self.id()]
            .1
            .write_grid_layout(render_state, grid_layout);
    }

    /// Update the grid layout for all instances, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// grid_layout: The grid layout to write
    fn write_grid_layout_collection(
        collection: &[(BuffersInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        grid_layout: &map::GridLayout,
    ) {
        Self::GridBackground(map::DataModeBackground::Light).write_grid_layout(
            collection,
            render_state,
            grid_layout,
        );
        Self::Sun.write_grid_layout(collection, render_state, grid_layout);
    }

    /// Sets the correct instance from the collection, returns the number of instance elements set
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_pass: The render pass to draw to
    fn set<'a>(
        &self,
        collection: &'a [(BuffersInstance, UniformsInstance); Self::COUNT],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> u32 {
        collection[self.id()].1.set(render_pass);
        return collection[self.id()].0.set(render_pass);
    }
}

/// Holds GPU buffers for one type of instance data
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
                    label: Some("Instance Buffer"),
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

/// Holds all of the uniforms for a single instance type
struct UniformsInstance {
    /// The buffer for the world to screen coordinates transform
    transform: wgpu::Buffer,
    /// The buffer for the color map data
    color_map: wgpu::Buffer,
    /// The buffer for the grid layout data
    grid_layout: wgpu::Buffer,
    /// The bind group for all uniforms
    bind_group: wgpu::BindGroup,
}

impl UniformsInstance {
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
                label: Some("Transform Uniform"),
                size: std::mem::size_of::<types::UniformTransform2D>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create color map buffer
        let color_map = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("ColorMap Uniform"),
                size: std::mem::size_of::<UniformColorMap>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        // Create grid layout buffer
        let grid_layout = render_state
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("GridLayout Uniform"),
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
    const _COUNT_VERTEX_HEXAGON: usize = 6;
    const _COUNT_VERTEX_RECTANGLE: usize = 4;
    const _COUNT_INDEX_BULK_HEXAGON: usize = 12;
    const _COUNT_INDEX_BULK_RECTANGLE: usize = 6;

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
    const fn vertices_hexagon() -> &'static [Self] {
        return &[
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
        ];
    }

    /// Generates the vertices for a rectangle
    const fn vertices_rectangle() -> &'static [Self] {
        return &[
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
        ];
    }

    /// Generates indices for the vertices for the bulk of a hexagon
    const fn indices_bulk_hexagon() -> &'static [u16] {
        return &[0, 1, 2, 2, 3, 0, 3, 5, 0, 3, 4, 5];
    }

    /// Generates indices for the vertices for the bulk of a rectangle
    const fn indices_bulk_rectangle() -> &'static [u16] {
        return &[0, 1, 2, 2, 3, 0];
    }
}
