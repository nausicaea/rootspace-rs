use nalgebra::Vector2;
use common::file_manipulation::FileError as RootFileError;
use common::resource_group::{ShaderGroup, TextureGroup, FontGroup};
use common::layout_group::MarginGroup;

pub struct SpeechBubble {
    pub relative_position_offset: Vector2<f32>,
    pub text_width: u32,
    pub margin: MarginGroup,
    pub font: FontGroup,
    pub text_shaders: ShaderGroup,
    pub rect_shaders: ShaderGroup,
    pub rect_textures: TextureGroup,
}

impl SpeechBubble {
    pub fn new(font_group: FontGroup, text_shaders: ShaderGroup, rect_shaders: ShaderGroup, rect_textures: TextureGroup) -> Self {
        SpeechBubble {
            relative_position_offset: Vector2::new(-0.5, 0.5),
            text_width: 100,
            margin: MarginGroup::new(5.0, 5.0, 5.0, 50.0),
            font: font_group,
            text_shaders: text_shaders,
            rect_shaders: rect_shaders,
            rect_textures: rect_textures,
        }
    }
}

pub struct Tooltip {
    pub relative_position_offset: Vector2<f32>,
    pub text_width: u32,
    pub margin: MarginGroup,
    pub font: FontGroup,
    pub text_shaders: ShaderGroup,
    pub rect_shaders: ShaderGroup,
    pub rect_textures: TextureGroup
}

impl Tooltip {
    pub fn new(font_group: FontGroup, text_shaders: ShaderGroup, rect_shaders: ShaderGroup, rect_textures: TextureGroup) -> Self {
        Tooltip {
            relative_position_offset: Vector2::new(-0.5, 0.5),
            text_width: 100,
            margin: MarginGroup::new(5.0, 5.0, 5.0, 50.0),
            font: font_group,
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
