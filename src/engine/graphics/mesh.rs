use glium::{Display, VertexBuffer, IndexBuffer};
use glium::vertex;
use glium::index;
use ecs::ComponentTrait;
use super::vertex::Vertex;

pub type Index = u16;

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

/// The `Mesh` encapsulates a vertex and an index buffer. In concert, they specify all vertices of
/// a 3D object.
pub struct Mesh {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<Index>,
}

impl Mesh {
    /// Create a new `Mesh`.
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[Index], primitive: index::PrimitiveType) -> Result<Self, MeshError> {
        Ok(Mesh {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: IndexBuffer::new(display, primitive, indices)?,
        })
    }
}

impl ComponentTrait for Mesh {}
