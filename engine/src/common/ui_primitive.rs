use glium::Display;
use rusttype::PositionedGlyph;
use rusttype::gpu_cache::Cache;
use components::model::Model;
use components::mesh::{MeshError, Mesh};
use components::material::Material;

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
        let mesh = Mesh::text(display, screen_dims, z_value, cache, glyphs, text_dims)?;

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

