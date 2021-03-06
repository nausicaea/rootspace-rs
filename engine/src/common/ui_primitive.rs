use glium::Display;
use nalgebra::{zero, Vector2, Vector3};
use rusttype::PositionedGlyph;
use rusttype::gpu_cache::Cache;
use common::resource_group::{ShaderGroup, TextureGroup};
use common::text_rendering::generate_vertices;
use common::factory::{ComponentFactory, FactoryError as RootFactoryError};
use components::model::Model;
use components::mesh::{BufferType, Mesh, MeshError as RootMeshError};
use components::material::Material;

/// A `UiPrimitive` encodes all data necessary to render the primitive to the display (vertices,
/// indices, material, uniforms).
pub struct UiPrimitive {
    pub model: Model,
    pub mesh: Mesh,
    pub material: Material,
    pub text_color: Vector3<f32>,
}

impl UiPrimitive {
    /// Creates a new `UiPrimitive`.
    pub fn new(model: Model, mesh: Mesh, material: Material, text_color: Vector3<f32>) -> Self {
        UiPrimitive {
            model: model,
            mesh: mesh,
            material: material,
            text_color: text_color,
        }
    }
    pub fn create_rectangle(
        display: &Display,
        factory: &mut ComponentFactory,
        center: Vector3<f32>,
        dims: Vector2<f32>,
        shaders: &ShaderGroup,
        textures: &TextureGroup,
    ) -> PrimResult {
        let rect_model = Model::new(center, zero(), Vector3::new(dims.x, dims.y, 1.0));
        let rect_mesh = Mesh::new_quad(display)?;
        let rect_material = factory.new_material(display, shaders, textures)?;

        Ok(UiPrimitive::new(
            rect_model,
            rect_mesh,
            rect_material,
            Vector3::new(0.0, 0.0, 0.0),
        ))
    }
    pub fn create_text(
        display: &Display,
        factory: &mut ComponentFactory,
        font_cache: &Cache,
        screen_dims: &Vector2<f32>,
        center: Vector3<f32>,
        dims: &Vector2<f32>,
        glyphs: &[PositionedGlyph],
        shaders: &ShaderGroup,
        text_color: Vector3<f32>,
    ) -> PrimResult {
        let text_model = Model::new(center, zero(), Vector3::new(1.0, 1.0, 1.0));
        let (vertices, indices, primitive) =
            generate_vertices(font_cache, screen_dims.as_ref(), dims.as_ref(), glyphs);
        let text_mesh = Mesh::new(display, &vertices, &indices, primitive, BufferType::Dynamic)?;
        let text_material = factory.new_material(display, shaders, &TextureGroup::empty())?;

        Ok(UiPrimitive::new(
            text_model,
            text_mesh,
            text_material,
            text_color,
        ))
    }
}

pub type PrimResult = Result<UiPrimitive, UiPrimitiveError>;

#[derive(Debug, Fail)]
pub enum UiPrimitiveError {
    #[fail(display = "{}", _0)] FactoryError(#[cause] RootFactoryError),
    #[fail(display = "{}", _0)] MeshError(#[cause] RootMeshError),
}

impl From<RootFactoryError> for UiPrimitiveError {
    fn from(value: RootFactoryError) -> Self {
        UiPrimitiveError::FactoryError(value)
    }
}

impl From<RootMeshError> for UiPrimitiveError {
    fn from(value: RootMeshError) -> Self {
        UiPrimitiveError::MeshError(value)
    }
}
