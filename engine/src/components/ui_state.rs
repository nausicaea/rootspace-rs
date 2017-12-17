use std::borrow::Cow;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use uuid::Uuid;
use rusttype::gpu_cache::Cache;
use glium::Display;
use glium::texture::{Texture2d, RawImage2d, UncompressedFloatFormat, MipmapsOption, ClientFormat, TextureCreationError};
use ecs::ComponentTrait;
use common::ui_element::UiElement;
use common::ui_styles::{Common, SpeechBubble};

pub struct UiState {
    pub elements: HashMap<Uuid, UiElement>,
    pub lifetimes: HashMap<Uuid, (Instant, Duration)>,
    pub font_cache_cpu: Cache<'static>,
    pub font_cache_gpu: Texture2d,
    pub common: Common,
    pub speech_bubble: SpeechBubble,
    pub dimensions: [u32; 2],
}

impl UiState {
    pub fn new(display: &Display, dimensions: &[u32; 2], hi_dpi_factor: f32, common: Common, speech_bubble: SpeechBubble) -> Result<Self, TextureCreationError> {
        let cache_width = dimensions[0] * hi_dpi_factor as u32;
        let cache_height = dimensions[1] * hi_dpi_factor as u32;
        let scale_tolerance = 0.1;
        let position_tolerance = 0.1;
        let cpu_cache = Cache::new(cache_width, cache_height, scale_tolerance, position_tolerance);
        let raw_tex = RawImage2d {
            data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
            width: cache_width,
            height: cache_height,
            format: ClientFormat::U8
        };
        let gpu_cache = Texture2d::with_format(display, raw_tex, UncompressedFloatFormat::U8, MipmapsOption::NoMipmap)?;

        Ok(UiState {
            elements: Default::default(),
            lifetimes: Default::default(),
            font_cache_cpu: cpu_cache,
            font_cache_gpu: gpu_cache,
            common: common,
            speech_bubble: speech_bubble,
            dimensions: *dimensions,
        })
    }
}

impl ComponentTrait for UiState {}
