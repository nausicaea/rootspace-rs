use glium::Display;
use glium::index;
use rusttype::{Rect, PositionedGlyph, point, vector};
use rusttype::gpu_cache::Cache;
use geometry::model::Model;
use graphics::vertex::Vertex;
use graphics::mesh::{MeshError, Mesh};
use graphics::material::Material;

#[derive(Debug, Fail)]
pub enum UiPrimitiveError {
    #[fail(display = "{}", _0)]
    MeshCreationError(#[cause] MeshError),
}

impl From<MeshError> for UiPrimitiveError {
    fn from(value: MeshError) -> Self {
        UiPrimitiveError::MeshCreationError(value)
    }
}

/// A `UiPrimitive` encodes all data necessary to render the primitive to the display (vertices,
/// indices, material, uniforms).
pub struct UiPrimitive {
    pub model: Model,
    pub mesh: Mesh,
    pub material: Material,
}

impl UiPrimitive {
    /// Creates a new `UiPrimitive`.
    pub fn new(model: Model, mesh: Mesh, material: Material) -> Self {
        UiPrimitive {
            model: model,
            mesh: mesh,
            material: material,
        }
    }
    /// Creates a new `UiPrimitive` that contains rendered text.
    pub fn new_text(display: &Display, screen_dims: &[u32; 2], z_value: f32, cache: &Cache, glyphs: &[PositionedGlyph], text_dims: &[f32; 2], model: Model, material: Material) -> Result<Self, UiPrimitiveError> {
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

        let mesh = Mesh::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)?;

        Ok(Self::new(model, mesh, material))
    }
    /// Creates a new `UiPrimitive` that contains a textured rectangle.
    pub fn new_rect(display: &Display, rect_dims: &[f32; 2], z_value: f32, model: Model, material: Material) -> Result<Self, UiPrimitiveError> {
        let min = [-rect_dims[0] / 2.0, -rect_dims[1] / 2.0];
        let max = [rect_dims[0] / 2.0, rect_dims[1] / 2.0];
        let mesh = Mesh::quad(display, &min, &max, z_value)?;

        Ok(Self::new(model, mesh, material))
    }
}

