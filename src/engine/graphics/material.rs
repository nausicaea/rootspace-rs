use std::io;
use std::path::{Path, PathBuf};
use glium::{Display, Program};
use glium::program;
use ecs::ComponentTrait;
use super::super::utilities::load_text_file;

#[derive(Debug, Fail)]
pub enum MaterialError {
    #[fail(display = "{}", _0)]
    ShaderError(#[cause] program::ProgramCreationError),
    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),
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

/// The `Material` represents an abstraction of a real-world material of an object.
pub struct Material {
    vs: PathBuf,
    fs: PathBuf,
    gs: Option<PathBuf>,
    /// Provides access to the shader program.
    pub shader: Program,
}

impl Material {
    /// Creates a new `Material` from multiple shader files.
    pub fn new(display: &Display, vs: &Path, fs: &Path, gs: Option<&Path>) -> Result<Self, MaterialError> {
        let vss = load_text_file(vs)?;
        let fss = load_text_file(fs)?;
        let gss = if let Some(gp) = gs {
            Some(load_text_file(gp)?)
        } else {
            None
        };

        Ok(Material {
            vs: vs.to_owned(),
            fs: fs.to_owned(),
            gs: gs.map(|gp| gp.to_owned()),
            shader: Program::from_source(display, &vss, &fss, gss.as_ref().map(|g| &**g))?,
        })
    }
}

impl ComponentTrait for Material {}
