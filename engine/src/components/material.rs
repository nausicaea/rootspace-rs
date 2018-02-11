//! The `material` module provides access to the `Material` component.

use std::rc::Rc;
use glium::{Display, Program, Texture2d};
use glium::program;
use glium::texture;
use common::file_manipulation::{load_text_file, load_image_file, FileError as RootFileError};
use common::resource_group::{ShaderGroup, TextureGroup};

/// The `Material` represents an abstraction of a real-world material of an object.
#[derive(Clone, Component)]
pub struct Material {
    /// Provides access to the shader program.
    pub shader: Rc<Program>,
    /// Provides access to the diffuse texture.
    pub diff_tex: Option<Rc<Texture2d>>,
    /// Provides access to the normal texture.
    pub norm_tex: Option<Rc<Texture2d>>,
    shader_origins: ShaderGroup,
    texture_origins: TextureGroup,
}

impl Material {
    /// Creates a new `Material` from multiple shader files.
    pub fn new(display: &Display, shaders: ShaderGroup, textures: TextureGroup) -> Result<Self, MaterialError> {
        let vss = load_text_file(&shaders.vertex)?;
        let fss = load_text_file(&shaders.fragment)?;
        let gss = match shaders.geometry {
            Some(ref gp) => Some(load_text_file(gp)?),
            None => None,
        };
        let dtt = match textures.diffuse {
            Some(ref dp) => {
                let di = load_image_file(dp)?;
                Some(Rc::new(Texture2d::new(display, di)?))
            },
            None => None,
        };
        let ntt = match textures.normal {
            Some(ref np) => {
                let ni = load_image_file(np)?;
                Some(Rc::new(Texture2d::new(display, ni)?))
            },
            None => None,
        };

        Ok(Material {
            shader: Rc::new(Program::from_source(display, &vss, &fss, gss.as_ref().map(|g| &**g))?),
            diff_tex: dtt,
            norm_tex: ntt,
            shader_origins: shaders,
            texture_origins: textures,
        })
    }
}

/// Operations with `Material` may fail. `MaterialError` describes those errors.
#[derive(Debug, Fail)]
pub enum MaterialError {
    #[fail(display = "{}", _0)]
    ShaderError(#[cause] program::ProgramCreationError),
    #[fail(display = "{}", _0)]
    FileError(#[cause] RootFileError),
    #[fail(display = "{}", _0)]
    TextureError(#[cause] texture::TextureCreationError),
}

impl From<program::ProgramCreationError> for MaterialError {
    fn from(value: program::ProgramCreationError) -> Self {
        MaterialError::ShaderError(value)
    }
}

impl From<RootFileError> for MaterialError {
    fn from(value: RootFileError) -> Self {
        MaterialError::FileError(value)
    }
}

impl From<texture::TextureCreationError> for MaterialError {
    fn from(value: texture::TextureCreationError) -> Self {
        MaterialError::TextureError(value)
    }
}
