use wgpu::util::DeviceExt;

use crate::render;

use super::Vertex;

/// Describes which primitive (square, hexagon, ect.) to draw
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PrimitiveType {
    /// Draw hexagons
    Hexagon,
    /// Draw rectangles
    Rectangle,
}

impl PrimitiveType {
    /// The number of different primitives
    pub(super) const COUNT: usize = 2;

    /// The id to find the primitive in the buffer list
    pub(super) fn id(&self) -> usize {
        return match self {
            Self::Hexagon => 0,
            Self::Rectangle => 1,
        };
    }

    /// Gets a list of all the different primitives
    pub(super) const fn all_primitives() -> &'static [Self; Self::COUNT] {
        return &[Self::Hexagon, Self::Rectangle];
    }

    /// Constructs a new pipeline matching the pipeline type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    pub(super) fn new(&self, render_state: &render::RenderState) -> BufferVertices {
        let (vertices, bulk_indices) = match self {
            Self::Hexagon => (Vertex::vertices_hexagon(), Vertex::indices_bulk_hexagon()),
            Self::Rectangle => (
                Vertex::vertices_rectangle(),
                Vertex::indices_bulk_rectangle(),
            ),
        };

        return BufferVertices::new(render_state, vertices, bulk_indices);
    }

    /// Constructs the primitive vertices for all the different primitive type
    ///
    /// # Parameters
    ///
    /// render_state: The render state to use for rendering
    pub(super) fn new_collection(
        render_state: &render::RenderState,
    ) -> [BufferVertices; Self::COUNT] {
        return Self::all_primitives()
            .iter()
            .map(|primitive| primitive.new(render_state))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }

    /// Sets the correct primitive from the collection, returns the number of indices set
    ///
    /// # Parameters
    ///
    /// collection: The full collection of primitives
    ///
    /// render_pass: The render pass to draw to
    pub(super) fn set<'a>(
        &self,
        collection: &'a [BufferVertices; Self::COUNT],
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> u32 {
        return collection[self.id()].set(render_pass);
    }
}

/// Holds GPU buffers for the vertex data to draw a single tile
#[derive(Debug)]
pub(super) struct BufferVertices {
    /// The buffer holding all four vertices of the tile
    vertices: wgpu::Buffer,
    /// The indices describing the triangles of the fill
    indices_bulk: wgpu::Buffer,
    /// The number of bulk indices
    count: u32,
}

impl BufferVertices {
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
