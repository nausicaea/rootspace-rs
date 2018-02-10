use std::fs::File;
use std::io::{Read, Error};
use std::path::Path;
use image;
use glium::texture::RawImage2d;

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
