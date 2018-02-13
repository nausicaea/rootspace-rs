///! The `mesh` module provides access to `Mesh`.

use glium::{Display, VertexBuffer, IndexBuffer};
use glium::vertex;
use glium::index;
use rusttype::{PositionedGlyph, point, vector, Rect};
use rusttype::gpu_cache::Cache;
use nalgebra::Vector2;
use common::vertex::Vertex;

/// The `Mesh` encapsulates a vertex and an index buffer. In concert, they specify all vertices of
/// a 3D object.
#[derive(Component)]
pub struct Mesh {
    /// Holds the vertex buffer object.
    pub vertices: VertexBuffer<Vertex>,
    /// Holds the index buffer object.
    pub indices: IndexBuffer<u16>,
}

impl Mesh {
    /// Creates a new `Mesh` component.
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[u16], primitive: index::PrimitiveType) -> Result<Self, MeshError> {
        Ok(Mesh {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: IndexBuffer::new(display, primitive, indices)?,
        })
    }
    /// Creates a new unit square.
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

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)
    }
    /// Creates a new unit cube.
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

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)
    }
    /// Creates a series of textured rectangles each with a glyph as texture.
    pub fn new_text(display: &Display, screen_dims: &Vector2<u32>, z_value: f32, cache: &Cache, glyphs: &[PositionedGlyph], text_dims: &Vector2<f32>) -> Result<Self, MeshError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let origin = point(-text_dims.x / 2.0, text_dims.y / 2.0);

        let mut quad_counter = 0;
        glyphs.iter().for_each(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let ndc_rect = Rect {
                    min: origin + vector(screen_rect.min.x as f32 / screen_dims.x as f32, -screen_rect.min.y as f32 / screen_dims.y as f32),
                    max: origin + vector(screen_rect.max.x as f32 / screen_dims.x as f32, -screen_rect.max.y as f32 / screen_dims.y as f32),
                };

                vertices.push(Vertex::new([ndc_rect.min.x, ndc_rect.max.y, z_value], [uv_rect.min.x, uv_rect.max.y], [0.0, 0.0, 1.0]));
                vertices.push(Vertex::new([ndc_rect.min.x, ndc_rect.min.y, z_value], [uv_rect.min.x, uv_rect.min.y], [0.0, 0.0, 1.0]));
                vertices.push(Vertex::new([ndc_rect.max.x, ndc_rect.min.y, z_value], [uv_rect.max.x, uv_rect.min.y], [0.0, 0.0, 1.0]));
                vertices.push(Vertex::new([ndc_rect.max.x, ndc_rect.max.y, z_value], [uv_rect.max.x, uv_rect.max.y], [0.0, 0.0, 1.0]));

                let stride = quad_counter * 4;
                indices.push(stride);
                indices.push(stride + 1);
                indices.push(stride + 2);
                indices.push(stride + 2);
                indices.push(stride + 3);
                indices.push(stride);
                quad_counter += 1;
            }
        });

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)
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
