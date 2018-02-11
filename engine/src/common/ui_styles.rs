use std::path::Path;
use rusttype::{FontCollection, Font};
use nalgebra::Vector2;
use common::file_manipulation::{load_binary_file, verify_accessible_file, FileError as RootFileError};
use common::resource_group::{ShaderGroup, TextureGroup};

pub struct Common {
    pub font: Font<'static>,
    pub font_scale: f32,
}

impl Common {
    pub fn new(font_path: &Path, font_scale: f32) -> Result<Self, UiStylesError> {
        verify_accessible_file(font_path)?;
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
    pub text_shaders: ShaderGroup,
    pub rect_shaders: ShaderGroup,
    pub rect_textures: TextureGroup,
}

impl SpeechBubble {
    pub fn new(text_shaders: ShaderGroup, rect_shaders: ShaderGroup, rect_textures: TextureGroup) -> Self {
        SpeechBubble {
            width: 100,
            margin_left: 5,
            margin_right: 5,
            margin_top: 5,
            margin_bottom: 50,
            relative_position_offset: Vector2::new(-0.5, 0.5),
            text_shaders: text_shaders,
            rect_shaders: rect_shaders,
            rect_textures: rect_textures,
        }
    }
}

pub struct Tooltip {
    pub width: u32,
    pub margin_left: u32,
    pub margin_right: u32,
    pub margin_top: u32,
    pub margin_bottom: u32,
    pub relative_position_offset: Vector2<f32>,
    pub text_shaders: ShaderGroup,
    pub rect_shaders: ShaderGroup,
    pub rect_textures: TextureGroup
}

impl Tooltip {
    pub fn new(text_shaders: ShaderGroup, rect_shaders: ShaderGroup, rect_textures: TextureGroup) -> Self {
        Tooltip {
            width: 100,
            margin_left: 5,
            margin_right: 5,
            margin_top: 5,
            margin_bottom: 50,
            relative_position_offset: Vector2::new(-0.5, 0.5),
            text_shaders: text_shaders,
            rect_shaders: rect_shaders,
            rect_textures: rect_textures,
        }
    }
}

#[derive(Debug, Fail)]
pub enum UiStylesError {
    #[fail(display = "{}", _0)]
    FileError(#[cause] RootFileError),
    #[fail(display = "Could not convert the FontCollection to a single Font.")]
    FontError,
}

impl From<RootFileError> for UiStylesError {
    fn from(value: RootFileError) -> Self {
        UiStylesError::FileError(value)
    }
}
