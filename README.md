A fast linear-time gaussian blur based on
<http://blog.ivank.net/fastest-gaussian-blur.html>.

This implementation was based on <https://github.com/fschutt/fastblur>.

These functions in-place blur a given slice of (presumably) image data, with any number of
channels and the given blur radius. Performance is roughly linear time, and uses a single
allocation for a backing store, of the same size as the input slice.

# Example
Blur an [`RgbImage`](<https://docs.rs/image/latest/image/type.RgbImage.html>)
```rust
fn blur_fast(rgb_image: &mut image::RgbImage, radius: f32) -> Result<(), blurslice::SliceSizeError> {
    let width = rgb_image.width() as usize;
    let height = rgb_image.width() as usize;
    let samples = rgb_image.as_flat_samples_mut();
    blurslice::gaussian_blur_bytes::<3>(samples.samples, width, height, radius)
}
```

# Changes:
  - Support any number of channels via const generics
  - No allocation for passes list generation, uses const generic stack array

## TODO:
  - [ ] Support arbitrary stride, for over-aligned data or vertical image sub-slices
  - [ ] Allow providing a backing store, to allow for zero-allocation execution

