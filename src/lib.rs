#![doc = include_str!("../README.md")]
mod fastblur;
mod from_byte_slice;

#[doc(inline)]
pub use fastblur::gaussian_blur;

#[doc(inline)]
pub use from_byte_slice::from_byte_slice;
pub use from_byte_slice::SliceSizeError;

/// Blur an image slice of packed pixel data
///
/// Blur a byte slice of at least `width` * `height` * `CHANNELS` length by `blur_radius`. This is
/// equivalent to calling [`from_byte_slice()`] followed by [`gaussian_blur`].
///
/// # Example
/// Blur an [`RgbImage`](<https://docs.rs/image/latest/image/type.RgbImage.html>)
/// ```
/// fn blur_fast(rgb_image: &mut image::RgbImage, radius: f32) -> Result<(), blurslice::SliceSizeError> {
///     let width = rgb_image.width() as usize;
///     let height = rgb_image.width() as usize;
///     let samples = rgb_image.as_flat_samples_mut();
///     blurslice::gaussian_blur_bytes::<3>(samples.samples, width, height, radius)
/// }
/// ```
/// See [`gaussian_blur`] for full documentation.
pub fn gaussian_blur_bytes<const CHANNELS: usize>(data: &mut [u8], width: usize, height: usize, blur_radius: f32) -> Result<(), SliceSizeError> {
    let channel_slice = from_byte_slice::from_byte_slice::<CHANNELS>(data)?;
    fastblur::gaussian_blur(channel_slice, width, height, blur_radius);
    Ok(())
}
