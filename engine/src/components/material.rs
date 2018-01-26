//! The `material` module provides access to the `Material` component.

use std::io;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use glium::{Display, Program, Texture2d};
use glium::program;
use glium::texture;
use image;
use common::file_manipulation::{load_text_file, load_image_file};

/// The `Material` represents an abstraction of a real-world material of an object.
#[derive(Clone, Component)]
pub struct Material {
    /// Provides access to the shader program.
    pub shader: Rc<Program>,
    /// Provides access to the diffuse texture.
    pub diff_tex: Option<Rc<Texture2d>>,
    /// Provides access to the normal texture.
    pub norm_tex: Option<Rc<Texture2d>>,
    vs_path: PathBuf,
    fs_path: PathBuf,
    gs_path: Option<PathBuf>,
    dt_path: Option<PathBuf>,
    nt_path: Option<PathBuf>,
}

impl Material {
    /// Creates a new `Material` from multiple shader files.
    pub fn new(display: &Display, vs: &Path, fs: &Path, gs: Option<&Path>, dt: Option<&Path>, nt: Option<&Path>) -> Result<Self, MaterialError> {
        let vss = load_text_file(vs)?;
        let fss = load_text_file(fs)?;
        let gss = match gs {
            Some(gp) => Some(load_text_file(gp)?),
            None => None,
        };
        let dtt = match dt {
            Some(dp) => {
                let di = load_image_file(dp)?;
                Some(Rc::new(Texture2d::new(display, di)?))
            },
            None => None,
        };
        let ntt = match nt {
            Some(np) => {
                let ni = load_image_file(np)?;
                Some(Rc::new(Texture2d::new(display, ni)?))
            },
            None => None,
        };

        Ok(Material {
            shader: Rc::new(Program::from_source(display, &vss, &fss, gss.as_ref().map(|g| &**g))?),
            diff_tex: dtt,
            norm_tex: ntt,
            vs_path: vs.to_path_buf(),
            fs_path: fs.to_path_buf(),
            gs_path: gs.map(|p| p.to_path_buf()),
            dt_path: dt.map(|p| p.to_path_buf()),
            nt_path: nt.map(|p| p.to_path_buf()),
        })
    }
}

/// Operations with `Material` may fail. `MaterialError` describes those errors.
#[derive(Debug, Fail)]
pub enum MaterialError {
    #[fail(display = "{}", _0)]
    ShaderError(#[cause] program::ProgramCreationError),
    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    ImageError(#[cause] image::ImageError),
    #[fail(display = "{}", _0)]
    TextureError(#[cause] texture::TextureCreationError),
}

impl From<program::ProgramCreationError> for MaterialError {
    fn from(value: program::ProgramCreationError) -> Self {
        MaterialError::ShaderError(value)
    }
}

impl From<io::Error> for MaterialError {
    fn from(value: io::Error) -> Self {
        MaterialError::IoError(value)
    }
}

impl From<image::ImageError> for MaterialError {
    fn from(value: image::ImageError) -> Self {
        MaterialError::ImageError(value)
    }
}

impl From<texture::TextureCreationError> for MaterialError {
    fn from(value: texture::TextureCreationError) -> Self {
        MaterialError::TextureError(value)
    }
}
