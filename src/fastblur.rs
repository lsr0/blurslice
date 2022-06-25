// Forked from <https://github.com/fschutt/fastblur>, in turn based on
// the article in <http://blog.ivank.net/fastest-gaussian-blur.html>

use std::cmp::min;

/// Blur an image slice of pixel arrays
///
/// In-place blur image provided image pixel data, with any number of channels. Will make a single
/// allocation for a backing buffer. Expects pixel data as a slice of CHANNELS sized arrays, for
/// use with a byte slice, use [`gaussian_blur_bytes`][super::gaussian_blur_bytes].
///
/// # Arguments
/// * `CHANNELS`: number of channels in the image data, e.g. 3 for RGB, 4 for RGBA, 1 for luminance
/// * `data`: pixel data, `width` x `height` in length
/// * `data` will be modified in-place
/// * `width`, `height`: in pixels
///
/// # Example
/// ```
/// let mut pixels = vec![[0xff, 0x00, 0xff], [0x00, 0xff, 0x00]];
/// blurslice::gaussian_blur(&mut pixels, 2, 1, 2.0);
/// assert!(pixels[0] != [0xff, 0x00, 0xff]);
/// ```
///
/// # Panics
/// If `data` is not at least as long as `width` * `height`
///
pub fn gaussian_blur<const CHANNELS: usize>(data: &mut [[u8; CHANNELS]], width: usize, height: usize, blur_radius: f32) {
    assert!(data.len() >= width * height);

    let boxes = create_box_gauss::<CHANNELS>(blur_radius);
    let mut backbuf = data.to_owned();

    for box_size in boxes.iter() {
        let radius = ((box_size - 1) / 2) as usize;
        box_blur(&mut backbuf, data, width, height, radius, radius, width);
    }
}

// TODO: Fully support strided inputs
#[allow(unused)]
fn gaussian_blur_stride<const CHANNELS: usize>(data: &mut [[u8; CHANNELS]], width: usize, height: usize, blur_radius: f32, stride: usize) {
    let boxes = create_box_gauss::<CHANNELS>(blur_radius);
    let mut backbuf = data.to_owned();

    for box_size in boxes.iter() {
        let radius = ((box_size - 1) / 2) as usize;
        box_blur(&mut backbuf, data, width, height, radius, radius, stride);
    }
}

#[inline]
fn create_box_gauss<const N: usize>(sigma: f32) -> [i32; N] {
    if sigma > 0.0 {
        let n_float = N as f32;

        let w_ideal = (12.0 * sigma * sigma / n_float).sqrt() + 1.0;
        let mut wl: i32 = w_ideal.floor() as i32;

        if wl % 2 == 0 {
            wl -= 1;
        };

        let wu = wl + 2;

        let wl_float = wl as f32;
        let m_ideal = (12.0 * sigma * sigma
            - n_float * wl_float * wl_float
            - 4.0 * n_float * wl_float
            - 3.0 * n_float)
            / (-4.0 * wl_float - 4.0);
        let m: usize = m_ideal.round() as usize;

        let mut sizes = [0; N];

        for (i, pass) in sizes.iter_mut().enumerate() {
            if i < m {
                *pass = wl;
            } else {
                *pass = wu;
            }
        }
        sizes
    } else {
        [1; N]
    }
}

#[inline]
fn box_blur<const CHANNELS: usize>(
    backbuf: &mut [[u8; CHANNELS]],
    frontbuf: &mut [[u8; CHANNELS]],
    width: usize,
    height: usize,
    blur_radius_horz: usize,
    blur_radius_vert: usize,
    stride: usize,
) {
    box_blur_horz(backbuf, frontbuf, width, height, blur_radius_horz, stride);
    box_blur_vert(frontbuf, backbuf, width, height, blur_radius_vert, stride);
}

#[inline]
fn box_blur_vert<const CHANNELS: usize>(
    backbuf: &[[u8; CHANNELS]],
    frontbuf: &mut [[u8; CHANNELS]],
    width: usize,
    height: usize,
    blur_radius: usize,
    stride: usize,
) {
    if blur_radius == 0 {
        frontbuf.copy_from_slice(backbuf);
        return;
    }

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;

    for i in 0..width {
        let col_start = i;
        let col_end = i + stride * (height - 1);
        let mut ti: usize = i;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius * width;

        let fv: [u8; CHANNELS] = backbuf[col_start];
        let lv: [u8; CHANNELS] = backbuf[col_end];

        let mut vals: [isize; CHANNELS] = [0; CHANNELS];
        for i in 0..CHANNELS {
            vals[i] = (blur_radius as isize + 1) * isize::from(fv[i]);
        }

        let get_top = |i: usize| {
            if i < col_start {
                fv
            } else {
                backbuf[i]
            }
        };

        let get_bottom = |i: usize| {
            if i > col_end {
                lv
            } else {
                backbuf[i]
            }
        };

        for j in 0..min(blur_radius, height) {
            let bb = backbuf[ti + j * stride];
            for i in 0..CHANNELS {
                vals[i] += isize::from(bb[i]);
            }
        }
        if blur_radius > height {
            for i in 0..CHANNELS {
                vals[i] += (blur_radius - height) as isize * isize::from(lv[i]);
            }
        }

        for _ in 0..min(height, blur_radius + 1) {
            let bb = get_bottom(ri);
            ri += width;
            for i in 0..CHANNELS {
                vals[i] += isize::from(bb[i]) - isize::from(fv[i]);
            }

            for i in 0..CHANNELS {
                frontbuf[ti][i] = round(vals[i] as f32 * iarr) as u8;
            }
            ti += width;
        }

        if height > blur_radius {
            for _ in (blur_radius + 1)..(height - blur_radius) {
                let bb1 = backbuf[ri];
                ri += width;
                let bb2 = backbuf[li];
                li += width;

                for i in 0..CHANNELS {
                    vals[i] += isize::from(bb1[i]) - isize::from(bb2[i]);
                }

                for i in 0..CHANNELS {
                    frontbuf[ti][i] = round(vals[i] as f32 * iarr) as u8;
                }
                ti += width;
            }

            for _ in 0..min(height - blur_radius - 1, blur_radius) {
                let bb = get_top(li);
                li += width;

                for i in 0..CHANNELS {
                    vals[i] += isize::from(lv[i]) - isize::from(bb[i]);
                }

                for i in 0..CHANNELS {
                    frontbuf[ti][i] = round(vals[i] as f32 * iarr) as u8;
                }
                ti += width;
            }
        }
    }
}

#[inline]
fn box_blur_horz<const CHANNELS: usize>(
    backbuf: &[[u8; CHANNELS]],
    frontbuf: &mut [[u8; CHANNELS]],
    width: usize,
    height: usize,
    blur_radius: usize,
    stride: usize,
) {
    if blur_radius == 0 {
        frontbuf.copy_from_slice(backbuf);
        return;
    }

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;

    for i in 0..height {
        let row_start: usize = i * stride;
        let row_end: usize = i * stride + width - 1;
        let mut ti: usize = i * stride;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius;

        let fv: [u8; CHANNELS] = backbuf[row_start];
        let lv: [u8; CHANNELS] = backbuf[row_end];

        let mut vals: [isize; CHANNELS] = [0; CHANNELS];
        for i in 0..CHANNELS {
            vals[i] = (blur_radius as isize + 1) * isize::from(fv[i]);
        }

        let get_left = |i: usize| {
            if i < row_start {
                fv
            } else {
                backbuf[i]
            }
        };

        let get_right = |i: usize| {
            if i > row_end {
                lv
            } else {
                backbuf[i]
            }
        };

        for j in 0..min(blur_radius, width) {
            let bb = backbuf[ti + j];
            for i in 0..CHANNELS {
                vals[i] += isize::from(bb[i]);
            }
        }
        if blur_radius > width {
            for i in 0..CHANNELS {
                vals[i] += (blur_radius - height) as isize * isize::from(lv[i]);
            }
        }

        for _ in 0..min(width, blur_radius + 1) {
            let bb = get_right(ri);
            ri += 1;
            for i in 0..CHANNELS {
                vals[i] += isize::from(bb[i]) - isize::from(fv[i]);
            }

            for i in 0..CHANNELS {
                frontbuf[ti][i] = round(vals[i] as f32 * iarr) as u8;
            }
            ti += 1;
        }

        if width > blur_radius {
            for _ in (blur_radius + 1)..(width - blur_radius) {
                let bb1 = backbuf[ri];
                ri += 1;
                let bb2 = backbuf[li];
                li += 1;

                for i in 0..CHANNELS {
                    vals[i] += isize::from(bb1[i]) - isize::from(bb2[i]);
                }

                for i in 0..CHANNELS {
                    frontbuf[ti][i] = round(vals[i] as f32 * iarr) as u8;
                }
                ti += 1;
            }

            for _ in 0..min(width - blur_radius - 1, blur_radius) {
                let bb = get_left(li);
                li += 1;

                for i in 0..CHANNELS {
                    vals[i] += isize::from(lv[i]) - isize::from(bb[i]);
                }

                for i in 0..CHANNELS {
                    frontbuf[ti][i] = round(vals[i] as f32 * iarr) as u8;
                }
                ti += 1;
            }
        }
    }
}

#[inline]
/// Source: https://stackoverflow.com/a/42386149/585725
fn round(mut x: f32) -> f32 {
    x += 12582912.0;
    x -= 12582912.0;
    x
}
