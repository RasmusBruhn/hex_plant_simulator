use wgpu::util::DeviceExt;

use crate::{map, render, types};

use super::{PipelineType, PrimitiveType};

/// Describes which mode to render in
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceMode {
    /// Instances for the sun data
    Sun,
    /// Instances for the background of the grid
    GridBackground(map::DataModeBackground),
}

impl InstanceMode {
    /// The number of different instance modes
    pub const COUNT: usize = 2;

    /// The id for the mode of the instance
    pub fn mode_id(&self) -> usize {
        return match self {
            Self::Sun => 0,
            Self::GridBackground(mode) => mode.id(),
        };
    }

    /// Retrieves the instance type for this mode
    pub fn get_type(&self) -> InstanceType {
        return match self {
            Self::Sun => InstanceType::Sun,
            Self::GridBackground(_) => InstanceType::GridBackground,
        };
    }

    /// The id to find the instance type in the instance list
    pub fn id(&self) -> usize {
        return self.get_type().id();
    }

    /// Gets a list of all the different instances
    ///
    /// # Parameters
    ///
    /// mode_background: The display mode for the grid background
    pub const fn all_instances(mode_background: map::DataModeBackground) -> [Self; Self::COUNT] {
        return [Self::Sun, Self::GridBackground(mode_background)];
    }

    /// Gets the pipeline used for this primitive
    pub(super) fn pipeline(&self) -> PipelineType {
        return match self {
            Self::Sun | Self::GridBackground(_) => PipelineType::Unicolor,
        };
    }

    /// Gets the data used for this instance
    ///
    /// # Parameters
    ///
    /// map: The map used to get data from
    pub(super) fn data<S: map::sun::Intensity>(&self, map: &map::Map<S>) -> Vec<map::InstanceTile> {
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
    pub(super) fn new<S: map::sun::Intensity>(
        &self,
        render_state: &render::RenderState,
        map: &map::Map<S>,
    ) -> (BufferInstance, UniformsInstance) {
        return (
            BufferInstance::new(render_state, &self.data(map)),
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
    /// mode_background: The display mode for the background of the tiles
    pub(super) fn new_collection<S: map::sun::Intensity>(
        render_state: &render::RenderState,
        map: &map::Map<S>,
        mode_background: map::DataModeBackground,
    ) -> [(BufferInstance, UniformsInstance); Self::COUNT] {
        return Self::all_instances(mode_background)
            .iter()
            .map(|instance| {
                return instance.new(render_state, map);
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
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
    pub(super) fn update<S: map::sun::Intensity>(
        &self,
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        map: &map::Map<S>,
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
    /// mode_background: The display mode for the background of the tiles
    pub(super) fn update_collection<S: map::sun::Intensity>(
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        map: &map::Map<S>,
        mode_background: map::DataModeBackground,
    ) {
        for instance in Self::all_instances(mode_background).iter() {
            instance.update(collection, render_state, map);
        }
    }

    /// Update the color map, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color_maps: The color maps for all modes
    pub(super) fn write_color_map(
        &self,
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        color_maps: &[Box<dyn types::ColorMap>],
    ) {
        collection[self.id()]
            .1
            .write_color_map(render_state, color_maps[self.mode_id()].as_ref());
    }

    /// Update the color maps for the entire collection of instances, this must be run once before the first rendering as it is not initialized
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_state: The render state to use for rendering
    ///
    /// color_maps: The color maps for all the instance types
    ///
    /// mode_background: The display mode for the background of the tiles
    pub(super) fn write_color_map_collection(
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        color_maps: &[Vec<Box<dyn types::ColorMap>>; Self::COUNT],
        mode_background: map::DataModeBackground,
    ) {
        for instance in Self::all_instances(mode_background).iter() {
            instance.write_color_map(collection, render_state, &color_maps[instance.id()]);
        }
    }
}

/// Describes which set of uniforms and primitives to use
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceType {
    /// Instances for the sun data
    Sun,
    /// Instances for the background of the grid
    GridBackground,
}

impl InstanceType {
    /// The number of different instance types
    pub const COUNT: usize = 2;

    /// The id to find the instance type in the instance list
    pub fn id(&self) -> usize {
        return match self {
            Self::Sun => 0,
            Self::GridBackground => 1,
        };
    }

    /// Gets a list of all the different instances
    pub const fn all_instances() -> &'static [Self; Self::COUNT] {
        return &[Self::Sun, Self::GridBackground];
    }

    /// Constructs a list of the color maps for all the instance types
    ///
    /// # Parameters
    ///
    /// sun: The color map for the sun
    ///
    /// background: The color map for all modes of the background of the grid
    pub fn new_color_map_collection(
        sun: Box<dyn types::ColorMap>,
        background: [Box<dyn types::ColorMap>; map::DataModeBackground::COUNT],
    ) -> [Vec<Box<dyn types::ColorMap>>; Self::COUNT] {
        return [vec![sun], background.into()];
    }

    /// Gets the primitive type used for this instance
    pub(super) fn primitive(&self) -> PrimitiveType {
        return match self {
            Self::Sun => PrimitiveType::Rectangle,
            Self::GridBackground => PrimitiveType::Hexagon,
        };
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
    pub(super) fn write_transform(
        &self,
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        transform: &types::Transform2D,
    ) {
        collection[self.id()]
            .1
            .write_transform(render_state, transform);
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
    pub(super) fn write_grid_layout(
        &self,
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
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
    pub(super) fn write_grid_layout_collection(
        collection: &[(BufferInstance, UniformsInstance); Self::COUNT],
        render_state: &render::RenderState,
        grid_layout: &map::GridLayout,
    ) {
        for instance in Self::all_instances().iter() {
            instance.write_grid_layout(collection, render_state, grid_layout);
        }
    }

    /// Sets the correct instance from the collection, returns the number of instance elements set
    ///
    /// # Parameters
    ///
    /// collection: The full collection of instances
    ///
    /// render_pass: The render pass to draw to
    pub(super) fn set<'a>(
        &self,
        collection: &'a [(BufferInstance, UniformsInstance); Self::COUNT],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> u32 {
        collection[self.id()].1.set(render_pass);
        return collection[self.id()].0.set(render_pass);
    }
}

/// Holds GPU buffers for one type of instance data
#[derive(Debug)]
pub(super) struct BufferInstance {
    /// The data for all instances
    buffer: wgpu::Buffer,
    /// The number of instances
    count: u32,
}

impl BufferInstance {
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
#[derive(Debug)]
pub(super) struct UniformsInstance {
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
                size: std::mem::size_of::<types::UniformColorMap>() as u64,
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
    fn write_color_map(&self, render_state: &render::RenderState, color_map: &dyn types::ColorMap) {
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
    pub(super) fn bind_group_layout(render_state: &render::RenderState) -> wgpu::BindGroupLayout {
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
