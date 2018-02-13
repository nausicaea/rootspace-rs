use std::fs::File;
use std::io::{Read, Error};
use std::path::Path;
use image;
use glium::texture::{Texture2d, RawImage2d};

/// Reads the specified file into a string. This function assumes that the specified path points to
/// an accessible file.
pub fn load_text_file(file_path: &Path) -> Result<String, FileError> {
    let mut buf = String::new();
    File::open(file_path)
        .and_then(|mut f| f.read_to_string(&mut buf))?;

    Ok(buf)
}

/// Reads the specified file into a vector of bytes. This function assumes that the specified path
/// points to an accessible file.
pub fn load_binary_file(file_path: &Path) -> Result<Vec<u8>, FileError> {
    let mut buf = Vec::new();
    File::open(file_path)
        .and_then(|mut f| f.read_to_end(&mut buf))?;

    Ok(buf)
}

/// Reads the specified file as an image. This function assumes that the specified path points to
/// an accessible file.
pub fn load_image_file(file_path: &Path) -> Result<RawImage2d<u8>, FileError> {
    let dyn_img = image::open(file_path)?;
    let rgba_img = dyn_img.to_rgba();
    let dimensions = rgba_img.dimensions();

    Ok(RawImage2d::from_raw_rgba_reversed(&rgba_img.into_raw(), dimensions))
}

/// Given a texture and a path, saves the texture data as image to a file.
pub fn save_texture(tex: &Texture2d, path: &Path) -> Result<(), FileError> {
    let img = tex.read::<RawImage2d<u8>>();
    let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(img.width, img.height, img.data)
        .ok_or(FileError::CannotCreateImageFromRaw)?;
    img_buf.save(path)?;

    Ok(())
}

/// Verifies that the specified path is a file, and that it is accessible by the user (i.e. it
/// exists and the current user has access permissions).
pub fn verify_accessible_file(path: &Path) -> Result<(), FileError> {
    if !path.exists() {
        Err(FileError::FileNotFound(format!("{}", path.display())))
    } else if !path.is_file() {
        Err(FileError::NotAFile(format!("{}", path.display())))
    } else {
        Ok(())
    }
}

#[derive(Debug, Fail)]
pub enum FileError {
    #[fail(display = "No such file or directory: '{}'", _0)]
    FileNotFound(String),
    #[fail(display = "Not a file: '{}'", _0)]
    NotAFile(String),
    #[fail(display = "Failed to create an image buffer from the raw texture data.")]
    CannotCreateImageFromRaw,
    #[fail(display = "{}", _0)]
    IoError(#[cause] Error),
    #[fail(display = "{}", _0)]
    ImageError(#[cause] image::ImageError),
}

impl From<Error> for FileError {
    fn from(value: Error) -> Self {
        FileError::IoError(value)
    }
}

impl From<image::ImageError> for FileError {
    fn from(value: image::ImageError) -> Self {
        FileError::ImageError(value)
    }
}
