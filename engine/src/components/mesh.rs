use glium::{Display, VertexBuffer, IndexBuffer};
use glium::vertex;
use glium::index;
use rusttype::{PositionedGlyph, point, vector, Rect};
use rusttype::gpu_cache::Cache;
use ecs::ComponentTrait;
use common::vertex::Vertex;

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
    pub indices: IndexBuffer<u16>,
}

impl Mesh {
    /// Creates a new `Mesh`.
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[u16], primitive: index::PrimitiveType) -> Result<Self, MeshError> {
        Ok(Mesh {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: IndexBuffer::new(display, primitive, indices)?,
        })
    }
    /// Creates a new rectangle.
    pub fn new_quad(display: &Display, min: &[f32; 2], max: &[f32; 2], z_value: f32) -> Result<Self, MeshError> {
        let vertices = vec![
            Vertex::new([min[0], max[1], z_value], [0.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([min[0], min[1], z_value], [0.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], min[1], z_value], [1.0, 0.0], [0.0, 0.0, 1.0]),
            Vertex::new([max[0], max[1], z_value], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)
    }
    /// Creates a series of textured rectangles each with a glyph as texture.
    pub fn new_text(display: &Display, screen_dims: &[u32; 2], z_value: f32, cache: &Cache, glyphs: &[PositionedGlyph], text_dims: &[f32; 2]) -> Result<Self, MeshError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let origin = point(-text_dims[0] / 2.0, text_dims[1] / 2.0);

        let mut quad_counter = 0;
        glyphs.iter().for_each(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let ndc_rect = Rect {
                    min: origin + vector(screen_rect.min.x as f32 / screen_dims[0] as f32, -screen_rect.min.y as f32 / screen_dims[1] as f32),
                    max: origin + vector(screen_rect.max.x as f32 / screen_dims[0] as f32, -screen_rect.max.y as f32 / screen_dims[1] as f32),
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

impl ComponentTrait for Mesh {}
