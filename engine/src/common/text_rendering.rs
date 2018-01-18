use std::borrow::Cow;
use glium::Rect;
use glium::texture::{Texture2d, ClientFormat, RawImage2d};
use rusttype::{PositionedGlyph, Font, Scale, point};
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use unicode_normalization::UnicodeNormalization;

/// Given a string of text, font parameters and a text width, generates a set of positioned glyphs.
/// TODO: Write a better word-wrapping algorithm based on [StackOverflow](https://stackoverflow.com/a/857770)
pub fn layout_paragraph<'a>(font: &Font<'a>, scale: f32, width: u32, text: &str) -> (Vec<PositionedGlyph<'a>>, [u32; 2]) {
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
pub fn layout_paragraph_cached<'a>(cache: &mut Cache<'a>, cache_tex: &Texture2d, font: &Font<'a>, scale: f32, width: u32, text: &str) -> Result<(Vec<PositionedGlyph<'a>>, [u32; 2]), CacheWriteErr> {
    let (glyphs, text_dims) = layout_paragraph(font, scale, width, text);

    enqueue_glyphs(cache, &glyphs);

    update_cache(cache, cache_tex)?;

    Ok((glyphs, text_dims))
}

/// Updates the font cache based on the supplied glyphs.
fn enqueue_glyphs<'a>(cache: &mut Cache<'a>, glyphs: &[PositionedGlyph<'a>]) {
    for glyph in glyphs {
        cache.queue_glyph(0, glyph.clone());
    }
}

/// Given an up-to-date font cache (CPU side), updates the specified texture (e.g. the GPU side).
fn update_cache(cache: &mut Cache, cache_tex: &Texture2d) -> Result<(), CacheWriteErr> {
    cache.cache_queued(|rect, data| {
        cache_tex.main_level().write(Rect {
            left: rect.min.x,
            bottom: rect.min.y,
            width: rect.width(),
            height: rect.height()
        }, RawImage2d {
            data: Cow::Borrowed(data),
            width: rect.width(),
            height: rect.height(),
            format: ClientFormat::U8
        });
    })
}
