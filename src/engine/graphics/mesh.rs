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
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<Index>,
}

impl Mesh {
    /// Creates a new `Mesh`.
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[Index], primitive: index::PrimitiveType) -> Result<Self, MeshError> {
        Ok(Mesh {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: IndexBuffer::new(display, primitive, indices)?,
        })
    }
    /// Creates a new unit square.
    pub fn quad(display: &Display, min: &[f32; 2], max: &[f32; 2], z_value: f32) -> Result<Self, MeshError> {
        let vertices = vec![
            Vertex::new([min[0], max[1], z_value], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([min[0], min[1], z_value], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], min[1], z_value], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], max[1], z_value], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)
    }
}

impl ComponentTrait for Mesh {}
