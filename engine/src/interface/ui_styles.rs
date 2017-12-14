use std::path::Path;
use std::io::Error;
use rusttype::{FontCollection, Font};
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
}

impl SpeechBubble {
    pub fn new(width: u32) -> Self {
        SpeechBubble {
            width: width,
        }
    }
}
