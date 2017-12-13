use std::path::Path;
use glium::Display;
use glium::index;
use rusttype::{Rect, PositionedGlyph, point};
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use super::super::graphics::vertex::Vertex;
use super::super::graphics::mesh::{MeshError, Mesh, Index};
use super::super::graphics::material::{MaterialError, Material};

#[derive(Debug, Fail)]
pub enum UiPrimitiveError {
    #[fail(display = "{}", _0)]
    CacheError(String),
    #[fail(display = "{}", _0)]
    MeshCreationError(#[cause] MeshError),
    #[fail(display = "{}", _0)]
    MaterialCreationError(#[cause] MaterialError),
}

impl From<CacheWriteErr> for UiPrimitiveError {
    fn from(value: CacheWriteErr) -> Self {
        use self::UiPrimitiveError::*;

        match value {
            CacheWriteErr::GlyphTooLarge => CacheError("At least one of the queued glyphs is too
                                                       big to fit into the cache, even if all other
                                                       glyphs are removed".into()),
            CacheWriteErr::NoRoomForWholeQueue => CacheError("Not all of the requested glyphs can
                                                             fit into the cache, even if the cache
                                                             is completely cleared before the
                                                             attempt".into()),
        }
    }
}

impl From<MeshError> for UiPrimitiveError {
    fn from(value: MeshError) -> Self {
        UiPrimitiveError::MeshCreationError(value)
    }
}

impl From<MaterialError> for UiPrimitiveError {
    fn from(value: MaterialError) -> Self {
        UiPrimitiveError::MaterialCreationError(value)
    }
}

/// A `UiPrimitive` encodes all data necessary to render the primitive to the display (vertices,
/// indices, material, uniforms).
pub struct UiPrimitive {
    mesh: Mesh,
    material: Material,
}

impl UiPrimitive {
    /// Creates a new `UiPrimitive` that contains rendered text.
    pub fn new_text(display: &Display, screen_dims: &[u32; 2], z_value: f32, vs: &Path, fs: &Path, cache: &Cache, glyphs: &[PositionedGlyph]) -> Result<Self, UiPrimitiveError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        glyphs.iter().enumerate().for_each(|(i, g)| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: point(screen_rect.min.x as f32 / screen_dims[0] as f32, -screen_rect.min.y as f32 / screen_dims[1] as f32),
                    max: point(screen_rect.max.x as f32 / screen_dims[0] as f32, -screen_rect.max.y as f32 / screen_dims[1] as f32),
                };

                vertices.push(Vertex::new([gl_rect.min.x, gl_rect.max.y, z_value], [uv_rect.min.x, uv_rect.max.y], [0.0, 0.0, 1.0]));
                vertices.push(Vertex::new([gl_rect.min.x, gl_rect.min.y, z_value], [uv_rect.min.x, uv_rect.min.y], [0.0, 0.0, 1.0]));
                vertices.push(Vertex::new([gl_rect.max.x, gl_rect.min.y, z_value], [uv_rect.max.x, uv_rect.min.y], [0.0, 0.0, 1.0]));
                vertices.push(Vertex::new([gl_rect.max.x, gl_rect.max.y, z_value], [uv_rect.max.x, uv_rect.max.y], [0.0, 0.0, 1.0]));

                indices.push(i as Index);
                indices.push(i as Index + 1);
                indices.push(i as Index + 2);
                indices.push(i as Index + 2);
                indices.push(i as Index + 3);
                indices.push(i as Index);
            }
        });

        Ok(UiPrimitive {
            mesh: Mesh::new(display, &vertices, &indices, index::PrimitiveType::TrianglesList)?,
            material: Material::new(display, vs, fs, None, None, None)?,
        })
    }
    ///// Creates a new `UiPrimitive` that contains a textured rectangle.
    // pub fn new_rect(display: &Display, screen_dims: &[u32; 2], z_value: f32, rect_dims: &[f32; 2], vs: &Path, fs: &Path) -> Result<Self, UiPrimitiveError> {
    //     Ok(UiPrimitive {
    //         mesh: Mesh::quad(display, &min, &max, z_value),
    //         material
    //     })
    // }
}

