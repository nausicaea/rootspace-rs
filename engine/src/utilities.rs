use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Error};
use std::path::Path;
use image;
use glium::Rect;
use glium::texture::{Texture2d, ClientFormat, RawImage2d};
use nalgebra::{Translation3, Vector3, UnitQuaternion, Rotation3, Isometry3, Affine3, Matrix4, U3,
               U1, norm, Real, zero, one};
use rusttype::{PositionedGlyph, Font, Scale, point};
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use unicode_normalization::UnicodeNormalization;

/// Reads the specified file into a string.
pub fn load_text_file(file_path: &Path) -> Result<String, Error> {
    let mut buf = String::new();
    File::open(file_path)
        .and_then(|mut f| f.read_to_string(&mut buf))?;

    Ok(buf)
}

/// Reads the specified file into a vector of bytes.
pub fn load_binary_file(file_path: &Path) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    File::open(file_path)
        .and_then(|mut f| f.read_to_end(&mut buf))?;

    Ok(buf)
}

/// Reads the specified file as an image.
pub fn load_image_file(file_path: &Path) -> Result<RawImage2d<u8>, image::ImageError> {
    let dyn_img = image::open(file_path)?;
    let rgba_img = dyn_img.to_rgba();
    let dimensions = rgba_img.dimensions();

    Ok(RawImage2d::from_raw_rgba_reversed(&rgba_img.into_raw(), dimensions))
}

/// Given a string of text, font parameters and a text width, generates a set of positioned glyphs.
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

fn enqueue_glyphs<'a>(cache: &mut Cache<'a>, glyphs: &[PositionedGlyph<'a>]) {
    for glyph in glyphs {
        cache.queue_glyph(0, glyph.clone());
    }
}

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

/// Decomposes an affine T*R*S matrix into their constituents, where T corresponds to the
/// translational component, R refers to a rotation, and S refers to non-uniform scaling (without
/// shear).
pub fn decompose_trs_matrix<N>(value: &Affine3<N>) -> (Isometry3<N>, Affine3<N>) where N: Real {
    // Obtain the translational component.
    let t = Translation3::from_vector(value.matrix().fixed_slice::<U3, U1>(0, 3).into_owned());

    // Obtain the non-uniform scaling component.
    let s = Vector3::new(norm(&value.matrix().column(0).into_owned()),
                             norm(&value.matrix().column(1).into_owned()),
                             norm(&value.matrix().column(2).into_owned()));

    // Obtain the rotational component.
    let mut r = value.matrix().fixed_slice::<U3, U3>(0, 0).into_owned();
    for i in 0..3 {
        let mut temp = r.column_mut(i);
        temp /= s[i];
    }

    let r = UnitQuaternion::from_rotation_matrix(&Rotation3::from_matrix_unchecked(r));
    let i = Isometry3::from_parts(t, r);

    let s = Affine3::from_matrix_unchecked(Matrix4::new(s.x, zero(), zero(), zero(),
                                                        zero(), s.y, zero(), zero(),
                                                        zero(), zero(), s.z, zero(),
                                                        zero(), zero(), zero(), one()));
    (i, s)
}

