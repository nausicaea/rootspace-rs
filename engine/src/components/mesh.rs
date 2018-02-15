///! The `mesh` module provides access to `Mesh`.

use glium::{Display, VertexBuffer, IndexBuffer};
use glium::vertex;
use glium::index;
use rusttype::PositionedGlyph;
use rusttype::gpu_cache::Cache;
use nalgebra::Vector2;
use common::vertex::Vertex;
use common::text_rendering::generate_vertices;

/// Determines the type of buffer used by the `Mesh`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferType {
    Static,
    Dynamic,
    Persistent,
    Immutable,
}

/// The `Mesh` encapsulates a vertex and an index buffer. In concert, they specify all vertices of
/// a 3D object.
#[derive(Component)]
pub struct Mesh {
    /// Holds the vertex buffer object.
    pub vertices: VertexBuffer<Vertex>,
    /// Holds the index buffer object.
    pub indices: IndexBuffer<u16>,
    /// Holds the vertex buffer type
    pub buffer_type: BufferType,
}

impl Mesh {
    /// Creates a new `Mesh` component.
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[u16], primitive: index::PrimitiveType, buffer_type: BufferType) -> Result<Self, MeshError> {
        let vertices = match buffer_type {
            BufferType::Static => VertexBuffer::new(display, vertices),
            BufferType::Dynamic => VertexBuffer::dynamic(display, vertices),
            BufferType::Persistent => VertexBuffer::persistent(display, vertices),
            BufferType::Immutable => VertexBuffer::immutable(display, vertices),
        }?;

        let indices = match buffer_type {
            BufferType::Static => IndexBuffer::new(display, primitive, indices),
            BufferType::Dynamic => IndexBuffer::dynamic(display, primitive, indices),
            BufferType::Persistent => IndexBuffer::persistent(display, primitive, indices),
            BufferType::Immutable => IndexBuffer::immutable(display, primitive, indices),
        }?;

        Ok(Mesh {
            vertices: vertices,
            indices: indices,
            buffer_type: buffer_type,
        })
    }
    /// Creates a new unit square with static buffers.
    pub fn new_quad(display: &Display) -> Result<Self, MeshError> {
        // Specifies the half of the width of the square.
        let hw = 0.5;
        let vertices = [
            Vertex::new([-hw, hw, 0.0], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([-hw, -hw, 0.0], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([hw, -hw, 0.0], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([hw, hw, 0.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ];
        let indices = [0, 1, 2, 2, 3, 0];

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList, BufferType::Static)
    }
    /// Creates a new unit cube with static buffers.
    pub fn new_cube(display: &Display) -> Result<Self, MeshError> {
        // Specifies half of the width of the cube.
        let hw = 0.5;
        let vertices = [
            // Front face
            Vertex::new([-hw, hw, hw], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([-hw, -hw, hw], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([hw, -hw, hw], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([hw, hw, hw], [1.0, 1.0], [0.0, 0.0, 1.0]),
            // Back face
            Vertex::new([hw, hw, -hw], [0.0, 1.0], [0.0, 0.0, -1.0]),
            Vertex::new([hw, -hw, -hw], [0.0, 0.0], [0.0, 0.0, -1.0]),
            Vertex::new([-hw, -hw, -hw], [1.0, 0.0], [0.0, 0.0, -1.0]),
            Vertex::new([-hw, hw, -hw], [1.0, 1.0], [0.0, 0.0, -1.0]),
            // Right face
            Vertex::new([hw, hw, hw], [0.0, 1.0], [1.0, 0.0, 0.0]),
            Vertex::new([hw, -hw, hw], [0.0, 0.0], [1.0, 0.0, 0.0]),
            Vertex::new([hw, -hw, -hw], [1.0, 0.0], [1.0, 0.0, 0.0]),
            Vertex::new([hw, hw, -hw], [1.0, 1.0], [1.0, 0.0, 0.0]),
            // Left face
            Vertex::new([-hw, hw, -hw], [0.0, 1.0], [-1.0, 0.0, 0.0]),
            Vertex::new([-hw, -hw, -hw], [0.0, 0.0], [-1.0, 0.0, 0.0]),
            Vertex::new([-hw, -hw, hw], [1.0, 0.0], [-1.0, 0.0, 0.0]),
            Vertex::new([-hw, hw, hw], [1.0, 1.0], [-1.0, 0.0, 0.0]),
            // Top face
            Vertex::new([-hw, hw, -hw], [0.0, 1.0], [0.0, 1.0, 0.0]),
            Vertex::new([-hw, hw, hw], [0.0, 0.0], [0.0, 1.0, 0.0]),
            Vertex::new([hw, hw, hw], [1.0, 0.0], [0.0, 1.0, 0.0]),
            Vertex::new([hw, hw, -hw], [1.0, 1.0], [0.0, 1.0, 0.0]),
            // Bottom face
            Vertex::new([-hw, -hw, hw], [0.0, 1.0], [0.0, -1.0, 0.0]),
            Vertex::new([-hw, -hw, -hw], [0.0, 0.0], [0.0, -1.0, 0.0]),
            Vertex::new([hw, -hw, -hw], [1.0, 0.0], [0.0, -1.0, 0.0]),
            Vertex::new([hw, -hw, hw], [1.0, 1.0], [0.0, -1.0, 0.0]),
        ];
        let indices = [
            0, 1, 2, 2, 3, 0,
            4, 5, 6, 6, 7, 4,
            8, 9, 10, 10, 11, 8,
            12, 13, 14, 14, 15, 12,
            16, 17, 18, 18, 19, 16,
            20, 21, 22, 22, 23, 20,
        ];

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList, BufferType::Static)
    }
    pub fn update(&mut self, vertices: &[Vertex], indices: &[u16]) {

    }
}

/// Operations with `Mesh` might fail. `MeshError` describes those errors.
#[derive(Debug, Fail)]
pub enum MeshError {
    #[fail(display = "{}", _0)]
    VertexBufferError(#[cause] vertex::BufferCreationError),
    #[fail(display = "{}", _0)]
    IndexBufferError(#[cause] index::BufferCreationError),
}

impl From<vertex::BufferCreationError> for MeshError {
    fn from(value: vertex::BufferCreationError) -> Self {
        MeshError::VertexBufferError(value)
    }
}

impl From<index::BufferCreationError> for MeshError {
    fn from(value: index::BufferCreationError) -> Self {
        MeshError::IndexBufferError(value)
    }
}
