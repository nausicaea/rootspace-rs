use std::path::{Path, PathBuf};
use common::file_manipulation::{verify_accessible_file, FileError};

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
    pub fn new(vertex: &Path, fragment: &Path, geometry: Option<&Path>) -> Result<Self, FileError> {
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
    pub fn new(diffuse: Option<&Path>, normal: Option<&Path>) -> Result<Self, FileError> {
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
