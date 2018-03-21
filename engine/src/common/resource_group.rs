use std::borrow::Cow;
use std::path::{Path, PathBuf};
use glium::Display;
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d, Texture2d,
                     TextureCreationError as RootTextureCreationError, UncompressedFloatFormat};
use nalgebra::Vector3;
use rusttype::{Font, FontCollection, Error as RootFontError};
use rusttype::gpu_cache::Cache;
use common::file_manipulation::{load_binary_file, verify_accessible_file,
                                FileError as RootFileError};

/// Encapsulates font data as a path to the font, the font scale and the font itself.
#[derive(Clone)]
pub struct FontGroup {
    /// Holds the path to the font file.
    pub path: PathBuf,
    /// Determines the scale used for the font.
    pub scale: f32,
    pub color: Vector3<f32>,
    /// Holds the actual font object.
    pub font: Font<'static>,
}

impl FontGroup {
    /// Creates a new `FontGroup` while ensuring that the supplied font file is accessible. Also
    /// loads the font from the file.
    pub fn new(path: &Path, scale: f32, color: Vector3<f32>) -> Result<Self, ResourceError> {
        verify_accessible_file(path)?;
        let font_data = load_binary_file(path)?;
        let collection = FontCollection::from_bytes(font_data)?;
        let font = collection.into_font()?;

        Ok(FontGroup {
            path: path.into(),
            scale: scale,
            color: color,
            font: font,
        })
    }
}

pub struct FontCacheGroup {
    pub cpu: Cache<'static>,
    pub gpu: Texture2d,
}

impl FontCacheGroup {
    pub fn new(
        display: &Display,
        dimensions: &[u32; 2],
        hi_dpi_factor: u32,
    ) -> Result<Self, ResourceError> {
        let cache_width = dimensions[0] * hi_dpi_factor;
        let cache_height = dimensions[1] * hi_dpi_factor;
        let scale_tolerance = 0.1;
        let position_tolerance = 0.1;
        let cpu_cache = Cache::new(
            cache_width,
            cache_height,
            scale_tolerance,
            position_tolerance,
        );
        let raw_tex = RawImage2d {
            data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
            width: cache_width,
            height: cache_height,
            format: ClientFormat::U8,
        };
        let gpu_cache = Texture2d::with_format(
            display,
            raw_tex,
            UncompressedFloatFormat::U8,
            MipmapsOption::NoMipmap,
        )?;

        Ok(FontCacheGroup {
            cpu: cpu_cache,
            gpu: gpu_cache,
        })
    }
}

/// Encapsulates a group of shaders as a set of paths to the individual shader source files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderGroup {
    /// Holds the path to the vertex shader.
    pub vertex: PathBuf,
    /// Holds the path to the fragment shader.
    pub fragment: PathBuf,
    /// Optionally holds the path to the geometry shader.
    pub geometry: Option<PathBuf>,
}

impl ShaderGroup {
    /// Creates a new `ShaderGroup` while ensuring the existence of the specified shader source
    /// files.
    pub fn new(
        vertex: &Path,
        fragment: &Path,
        geometry: Option<&Path>,
    ) -> Result<Self, ResourceError> {
        verify_accessible_file(vertex)?;
        verify_accessible_file(fragment)?;
        if let Some(geom) = geometry {
            verify_accessible_file(geom)?;
        }

        Ok(ShaderGroup {
            vertex: vertex.into(),
            fragment: fragment.into(),
            geometry: geometry.map(|p| p.into()),
        })
    }
}

/// Encapsulates a group of textures as a set of paths to the individual texture files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextureGroup {
    /// Optionally holds the path to the diffuse texture file.
    pub diffuse: Option<PathBuf>,
    /// Optionally holds the path to the normal map file.
    pub normal: Option<PathBuf>,
}

impl TextureGroup {
    /// Creates a new `TextureGroup` while ensuring the existence of the specified texture files.
    pub fn new(diffuse: Option<&Path>, normal: Option<&Path>) -> Result<Self, ResourceError> {
        if let Some(diff) = diffuse {
            verify_accessible_file(diff)?;
        }
        if let Some(norm) = normal {
            verify_accessible_file(norm)?;
        }

        Ok(TextureGroup {
            diffuse: diffuse.map(|p| p.into()),
            normal: normal.map(|p| p.into()),
        })
    }
    /// Creates a new, empty `TextureGroup` that does not contain any textures.
    pub fn empty() -> Self {
        TextureGroup {
            diffuse: None,
            normal: None,
        }
    }
}

#[derive(Debug, Fail)]
pub enum ResourceError {
    #[fail(display = "{}", _0)] FileError(#[cause] RootFileError),
    #[fail(display = "{}", _0)] TextureCreationError(#[cause] RootTextureCreationError),
    #[fail(display = "{}", _0)] FontError(#[cause] RootFontError),
}

impl From<RootFileError> for ResourceError {
    fn from(value: RootFileError) -> Self {
        ResourceError::FileError(value)
    }
}

impl From<RootTextureCreationError> for ResourceError {
    fn from(value: RootTextureCreationError) -> Self {
        ResourceError::TextureCreationError(value)
    }
}

impl From<RootFontError> for ResourceError {
    fn from(value: RootFontError) -> Self {
        ResourceError::FontError(value)
    }
}
