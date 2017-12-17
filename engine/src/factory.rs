use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use glium::{Display, Program, Texture2d};
use glium::{program, texture};
use image;
use utilities::{load_text_file, load_image_file};

#[derive(Debug, Fail)]
pub enum FactoryError {
    #[fail(display = "{}", _0)]
    ShaderError(#[cause] program::ProgramCreationError),
    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    ImageError(#[cause] image::ImageError),
    #[fail(display = "{}", _0)]
    TextureError(#[cause] texture::TextureCreationError),
}

impl From<program::ProgramCreationError> for FactoryError {
    fn from(value: program::ProgramCreationError) -> Self {
        FactoryError::ShaderError(value)
    }
}

impl From<io::Error> for FactoryError {
    fn from(value: io::Error) -> Self {
        FactoryError::IoError(value)
    }
}

impl From<image::ImageError> for FactoryError {
    fn from(value: image::ImageError) -> Self {
        FactoryError::ImageError(value)
    }
}

impl From<texture::TextureCreationError> for FactoryError {
    fn from(value: texture::TextureCreationError) -> Self {
        FactoryError::TextureError(value)
    }
}

pub struct ComponentFactory {
    resource_path: PathBuf,
}

impl ComponentFactory {
    pub fn new(resource_path: &Path) -> Self {
        ComponentFactory {
            resource_path: resource_path.to_owned(),
        }
    }
    pub fn new_material(&mut self, display: &Display, vs: &Path, fs: &Path, gs: Option<&Path>, dt: Option<&Path>, nt: Option<&Path>) -> Result<(), FactoryError> {
        let vss = load_text_file(vs)?;
        let fss = load_text_file(fs)?;
        let gss = match gs {
            Some(gp) => Some(load_text_file(gp)?),
            None => None,
        };
        let shader = Rc::new(Program::from_source(display, &vss, &fss, gss.as_ref().map(|g| &**g))?);
        let dt = match dt {
            Some(dp) => {
                let di = load_image_file(dp)?;
                Some(Rc::new(Texture2d::new(display, di)?))
            },
            None => None,
        };
        let nt = match nt {
            Some(np) => {
                let ni = load_image_file(np)?;
                Some(Rc::new(Texture2d::new(display, ni)?))
            },
            None => None,
        };

        Ok(())
    }
}
