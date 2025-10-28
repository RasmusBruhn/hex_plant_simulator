use crate::{map, render};

use super::{UniformsInstance, Vertex};

/// Describes which pipeline to use
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PipelineType {
    /// All object are rendered with a uniform color from a color map
    Unicolor,
}

impl PipelineType {
    /// The number of different pipelines
    pub(super) const COUNT: usize = 1;

    /// The id to find the pipeline in the pipeline list
    pub(super) fn id(&self) -> usize {
        return match self {
            Self::Unicolor => 0,
        };
    }

    /// Gets a list of all the different pipelines
    pub(super) const fn all_pipelines() -> &'static [Self; Self::COUNT] {
        return &[Self::Unicolor];
    }

    /// Constructs a new pipeline matching the pipeline type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    pub(super) fn new(&self, render_state: &render::RenderState) -> Pipeline {
        let shader = match self {
            Self::Unicolor => wgpu::include_wgsl!("../shaders/unicolor.wgsl"),
        };

        return Pipeline::new(render_state, shader);
    }

    /// Constructs the pipelines for all the different pipeline type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    pub(super) fn new_collection(render_state: &render::RenderState) -> [Pipeline; Self::COUNT] {
        return Self::all_pipelines()
            .iter()
            .map(|pipeline| pipeline.new(render_state))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }

    /// Sets the correct pipeline from the collection
    ///
    /// # Parameters
    ///
    /// collection: The full collection of pipelines
    ///
    /// render_pass: The render pass to draw to
    pub(super) fn set<'a>(
        &self,
        collection: &'a [Pipeline; Self::COUNT],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        collection[self.id()].set(render_pass);
    }
}

/// Holds all render pipelines for a single pipeline type
#[derive(Debug)]
pub(super) struct Pipeline {
    /// The render pipeline for the fill
    fill: wgpu::RenderPipeline,
}

impl Pipeline {
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
