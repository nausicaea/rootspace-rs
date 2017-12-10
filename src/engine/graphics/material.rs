use std::io;
use std::path::{Path, PathBuf};
use glium::{Display, Program, Texture2d};
use glium::program;
use glium::texture;
use image;
use ecs::ComponentTrait;
use super::super::utilities::{load_text_file, load_image_file};

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

/// The `Material` represents an abstraction of a real-world material of an object.
pub struct Material {
    vs: PathBuf,
    fs: PathBuf,
    gs: Option<PathBuf>,
    /// Provides access to the shader program.
    pub shader: Program,
    pub diff_tex: Option<Texture2d>,
    pub norm_tex: Option<Texture2d>,
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
        let dt = match dt {
            Some(dp) => {
                let di = load_image_file(dp)?;
                Some(Texture2d::new(display, di)?)
            },
            None => None,
        };
        let nt = match nt {
            Some(np) => {
                let ni = load_image_file(np)?;
                Some(Texture2d::new(display, ni)?)
            },
            None => None,
        };

        Ok(Material {
            vs: vs.to_owned(),
            fs: fs.to_owned(),
            gs: gs.map(|gp| gp.to_owned()),
            shader: Program::from_source(display, &vss, &fss, gss.as_ref().map(|g| &**g))?,
            diff_tex: dt,
            norm_tex: nt,
        })
    }
    pub fn reload_shader(&mut self, display: &Display) -> Result<(), MaterialError> {
        let vss = load_text_file(&self.vs)?;
        let fss = load_text_file(&self.fs)?;
        let gss = match self.gs {
            Some(ref gp) => Some(load_text_file(gp)?),
            None => None,
        };

        self.shader = Program::from_source(display, &vss, &fss, gss.as_ref().map(|g| &**g))?;

        Ok(())
    }
}

impl ComponentTrait for Material {}
