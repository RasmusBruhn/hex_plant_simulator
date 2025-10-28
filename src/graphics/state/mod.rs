use super::{
    BufferInstance, BufferVertices, InstanceMode, InstanceType, Pipeline, PipelineType,
    PrimitiveType, Settings, UniformsInstance,
};
use crate::{map, render};

mod state_render;

/// A complete state for rendering
pub struct State {
    /// All of the settings for rendering
    settings: Settings,
    /// All pipelines used for rendering
    pipelines: [Pipeline; PipelineType::COUNT],
    /// All vertex buffers
    primitives: [BufferVertices; PrimitiveType::COUNT],
    /// All instance data both buffers and uniforms
    instances: [(BufferInstance, UniformsInstance); InstanceType::COUNT],
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
        let instances = InstanceMode::new_collection(render_state, map, settings.mode_background);

        let mut object = Self {
            settings,
            pipelines,
            primitives,
            instances,
        };
        object.settings_changed(render_state);

        return object;
    }

    /// Retrieves a reference to the settings
    pub fn get_settings(&self) -> &Settings {
        return &self.settings;
    }

    /// Sets the settings
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    ///
    /// settings: The new settings to set
    pub fn set_settings(&mut self, render_state: &render::RenderState, settings: Settings) {
        self.settings = settings;
        self.settings_changed(render_state);
    }

    /// Run when the settings has changed to update the internal state
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    fn settings_changed(&mut self, render_state: &render::RenderState) {
        InstanceMode::write_color_map_collection(
            &self.instances,
            render_state,
            &self.settings.color_maps,
            self.settings.mode_background,
        );
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
        InstanceMode::update_collection(
            &self.instances,
            render_state,
            map,
            self.settings.mode_background,
        );
    }
}
