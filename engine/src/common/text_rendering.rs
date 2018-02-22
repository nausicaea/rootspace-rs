use std::borrow::Cow;
use glium::Rect;
use glium::texture::{ClientFormat, RawImage2d, Texture2d};
use glium::index::PrimitiveType;
use rusttype::{point, vector, Font, PositionedGlyph, Rect as RusttypeRect, Scale};
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use unicode_normalization::UnicodeNormalization;
use common::vertex::Vertex;

/// Given a string of text, font parameters and a text width, generates a set of positioned glyphs.
/// TODO: Write a better word-wrapping algorithm based on [StackOverflow](https://stackoverflow.com/a/857770)
pub fn layout_paragraph<'a>(
    font: &Font<'a>,
    scale: f32,
    width: u32,
    text: &str,
) -> (Vec<PositionedGlyph<'a>>, [u32; 2]) {
    let mut result = Vec::new();
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let caret_origin = caret;
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            if let '\n' = c {
                caret = point(0.0, caret.y + advance_height);
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }

    let height = (caret.y - caret_origin.y + advance_height).ceil() as u32;

    (result, [width, height])
}

/// Layouts a paragraph of text using the GPU cache.
pub fn layout_paragraph_cached<'a>(
    cache: &mut Cache<'a>,
    cache_tex: &Texture2d,
    font: &Font<'a>,
    scale: f32,
    width: u32,
    text: &str,
) -> Result<(Vec<PositionedGlyph<'a>>, [u32; 2]), TextRenderError> {
    let (glyphs, text_dims) = layout_paragraph(font, scale, width, text);

    enqueue_glyphs(cache, &glyphs);

    update_cache(cache, cache_tex)?;

    Ok((glyphs, text_dims))
}

/// Given a set of glyphs, generates the vertices where every glyph is represented as a textured
/// rectangle.
pub fn generate_vertices(
    cache: &Cache,
    screen_dims: &[f32; 2],
    text_dims: &[f32; 2],
    glyphs: &[PositionedGlyph],
) -> (Vec<Vertex>, Vec<u16>, PrimitiveType) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let origin = point(-text_dims[0] / 2.0, text_dims[1] / 2.0);

    let mut quad_counter = 0;
    glyphs.iter().for_each(|g| {
        if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
            let ndc_rect = RusttypeRect {
                min: origin
                    + vector(
                        screen_rect.min.x as f32 / screen_dims[0],
                        -screen_rect.min.y as f32 / screen_dims[1],
                    ),
                max: origin
                    + vector(
                        screen_rect.max.x as f32 / screen_dims[0],
                        -screen_rect.max.y as f32 / screen_dims[1],
                    ),
            };

            vertices.push(Vertex::new(
                [ndc_rect.min.x, ndc_rect.max.y, 0.0],
                [uv_rect.min.x, uv_rect.max.y],
                [0.0, 0.0, 1.0],
            ));
            vertices.push(Vertex::new(
                [ndc_rect.min.x, ndc_rect.min.y, 0.0],
                [uv_rect.min.x, uv_rect.min.y],
                [0.0, 0.0, 1.0],
            ));
            vertices.push(Vertex::new(
                [ndc_rect.max.x, ndc_rect.min.y, 0.0],
                [uv_rect.max.x, uv_rect.min.y],
                [0.0, 0.0, 1.0],
            ));
            vertices.push(Vertex::new(
                [ndc_rect.max.x, ndc_rect.max.y, 0.0],
                [uv_rect.max.x, uv_rect.max.y],
                [0.0, 0.0, 1.0],
            ));

            let stride = quad_counter * 4;
            indices.push(stride);
            indices.push(stride + 1);
            indices.push(stride + 2);
            indices.push(stride + 2);
            indices.push(stride + 3);
            indices.push(stride);
            quad_counter += 1;
        }
    });

    (vertices, indices, PrimitiveType::TrianglesList)
}

/// Updates the font cache based on the supplied glyphs.
fn enqueue_glyphs<'a>(cache: &mut Cache<'a>, glyphs: &[PositionedGlyph<'a>]) {
    for glyph in glyphs {
        cache.queue_glyph(0, glyph.clone());
    }
}

/// Given an up-to-date font cache (CPU side), updates the specified texture (e.g. the GPU side).
fn update_cache(cache: &mut Cache, cache_tex: &Texture2d) -> Result<(), TextRenderError> {
    cache.cache_queued(|rect, data| {
        cache_tex.main_level().write(
            Rect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height(),
            },
            RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: ClientFormat::U8,
            },
        );
    })?;

    Ok(())
}

#[derive(Debug, Fail)]
pub enum TextRenderError {
    #[fail(display = "At least one of the queued glyphs is too big to fit into the cache, even if all other glyphs are removed")]
    GlyphTooLarge,
    #[fail(display = "Not all of the requested glyphs can fit into the cache, even if the cache is completely cleared before the attempt")]
    NoRoomForWholeQueue,
}

impl From<CacheWriteErr> for TextRenderError {
    fn from(value: CacheWriteErr) -> Self {
        match value {
            CacheWriteErr::GlyphTooLarge => TextRenderError::GlyphTooLarge,
            CacheWriteErr::NoRoomForWholeQueue => TextRenderError::NoRoomForWholeQueue,
        }
    }
}
