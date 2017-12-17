use std::path::{PathBuf, Path};
use std::io::Error;
use rusttype::{FontCollection, Font};
use nalgebra::Vector2;
use utilities::load_binary_file;

#[derive(Debug, Fail)]
pub enum UiStylesError {
    #[fail(display = "{}", _0)]
    IoError(#[cause] Error),
    #[fail(display = "Could not convert the FontCollection to a single Font.")]
    FontError,
}

impl From<Error> for UiStylesError {
    fn from(value: Error) -> Self {
        UiStylesError::IoError(value)
    }
}

pub struct Common {
    pub font: Font<'static>,
    pub font_scale: f32,
}

impl Common {
    pub fn new(font_path: &Path, font_scale: f32) -> Result<Self, UiStylesError> {
        let font_data = load_binary_file(font_path)?;
        let collection = FontCollection::from_bytes(font_data);

        Ok(Common {
            font: collection.into_font().ok_or(UiStylesError::FontError)?,
            font_scale: font_scale,
        })
    }
}

pub struct SpeechBubble {
    pub width: u32,
    pub margin_left: u32,
    pub margin_right: u32,
    pub margin_top: u32,
    pub margin_bottom: u32,
    pub relative_position_offset: Vector2<f32>,
    pub text_vertex_shader: PathBuf,
    pub text_fragment_shader: PathBuf,
    pub rect_vertex_shader: PathBuf,
    pub rect_fragment_shader: PathBuf,
    pub rect_diffuse_texture: PathBuf,
}

impl SpeechBubble {
    pub fn new(text_vs: &Path, text_fs: &Path, rect_vs: &Path, rect_fs: &Path, rect_difftex: &Path) -> Self {
        SpeechBubble {
            width: 100,
            margin_left: 5,
            margin_right: 5,
            margin_top: 5,
            margin_bottom: 50,
            relative_position_offset: Vector2::new(-0.5, 0.5),
            text_vertex_shader: text_vs.to_owned(),
            text_fragment_shader: text_fs.to_owned(),
            rect_vertex_shader: rect_vs.to_owned(),
            rect_fragment_shader: rect_fs.to_owned(),
            rect_diffuse_texture: rect_difftex.to_owned(),
        }
    }
}
