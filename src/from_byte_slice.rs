/// Convert a u8 slice into a slice of pixel arrays for [`gaussian_blur`]
///
/// Slice length must be an exact multiple of CHANNELS, else the function will fail with
/// [`SliceSizeError`]
///
/// # Example
/// ```
/// let mut magenta_pixel_bytes = vec![0xff, 0x00, 0xff, 0xff, 0x00, 0xff];
/// let pixels_res = blurslice::from_byte_slice::<3>(&mut magenta_pixel_bytes);
/// assert!(pixels_res.is_ok() && pixels_res.unwrap().len() == 2);
/// ```
///
/// [`gaussian_blur`]: super::gaussian_blur
pub fn from_byte_slice<const CHANNELS: usize>(slice: &mut [u8]) -> Result<&mut [[u8; CHANNELS]], SliceSizeError> {
    let pixel_count = slice.len() / CHANNELS;
    let expected = pixel_count * CHANNELS;
    if slice.len() != expected {
        return Err(SliceSizeError{expected, actual: slice.len(), channels: CHANNELS});
    }

    // Replace with https://doc.rust-lang.org/std/primitive.slice.html#method.as_chunks
    // when stabilised
    let as_pixels: &mut [[u8; CHANNELS]] = unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() / CHANNELS) };
    Ok(as_pixels)
}

/// Indicates given slice length doesn't match channel count
#[derive (Debug, PartialEq, Eq)]
pub struct SliceSizeError {
    pub expected: usize,
    pub actual: usize,
    pub channels: usize,
}

impl std::error::Error for SliceSizeError { }

impl std::fmt::Display for SliceSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "incorrect u8 slice length {len} for {channels} channel image, expected {expected} or {expected_above}",
               len = self.actual, channels = self.channels, expected = self.expected, expected_above = self.expected + self.channels)
    }
}


#[test]
fn test_successful_byte_slice() {
    let expected_results_rgb: Vec<(Vec<u8>, Result<Vec<[u8; 3]>, SliceSizeError>)> = vec![
        (vec![0xff, 0xb0, 0x00],              Ok(vec![[0xff, 0xb0, 0x00]])),
        (vec![],                              Ok(vec![])),
        (vec![0xff, 0xb0, 0x00, 0x01],       Err(SliceSizeError{expected: 3, actual: 4, channels: 3})),
        (vec![0xff, 0xb0, 0x00, 0x01, 0x02], Err(SliceSizeError{expected: 3, actual: 5, channels: 3})),
        (vec![0xff],                         Err(SliceSizeError{expected: 0, actual: 1, channels: 3})),
        (vec![0xff, 0xb0],                   Err(SliceSizeError{expected: 0, actual: 2, channels: 3})),
    ];

    for expected in expected_results_rgb {
        let mut mutable = expected.0.clone();
        let result = from_byte_slice::<3>(&mut mutable);
        let mapped = result.map(|r| -> Vec<_> { r.to_owned() });
        assert_eq!(expected.1, mapped);
    }

    let expected_results_rgba: Vec<(Vec<u8>, Result<Vec<[u8; 4]>, SliceSizeError>)> = vec![
        (vec![0xff, 0xb0, 0x00, 0x01],        Ok(vec![[0xff, 0xb0, 0x00, 0x01]])),
        (vec![],                              Ok(vec![])),
        (vec![0xff, 0xb0, 0x00, 0x01, 0x02], Err(SliceSizeError{expected: 4, actual: 5, channels: 4})),
        (vec![0xff],                         Err(SliceSizeError{expected: 0, actual: 1, channels: 4})),
        (vec![0xff, 0xb0],                   Err(SliceSizeError{expected: 0, actual: 2, channels: 4})),
        (vec![0xff, 0xb0, 0x00],             Err(SliceSizeError{expected: 0, actual: 3, channels: 4})),
        (vec![0xff, 0xb0, 0x00, 0x01, 0xff, 0xb0, 0x00, 0x01, 0x50, 0x99, 0x44, 0x34],
      Ok(vec![[0xff, 0xb0, 0x00, 0x01], [0xff, 0xb0, 0x00, 0x01], [0x50, 0x99, 0x44, 0x34]])),
    ];

    for expected in expected_results_rgba {
        let mut mutable = expected.0.clone();
        let result = from_byte_slice::<4>(&mut mutable);
        let mapped = result.map(|r| -> Vec<_> { r.to_owned() });
        assert_eq!(expected.1, mapped);
    }

    let expected_results_luma: Vec<(Vec<u8>, Result<Vec<[u8; 1]>, SliceSizeError>)> = vec![
        (vec![0xff],                          Ok(vec![[0xff]])),
        (vec![0xff, 0xb0, 0x00, 0x01],        Ok(vec![[0xff], [0xb0], [0x00], [0x01]])),
        (vec![],                              Ok(vec![])),
    ];

    for expected in expected_results_luma {
        let mut mutable = expected.0.clone();
        let result = from_byte_slice::<1>(&mut mutable);
        let mapped = result.map(|r| -> Vec<_> { r.to_owned() });
        assert_eq!(expected.1, mapped);
    }
}
