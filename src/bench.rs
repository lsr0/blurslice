#![feature(test)]

extern crate test;

use crate::fastblur;

#[bench]
fn bench_blur_image(b: &mut test::Bencher) {

    use fastblur::gaussian_blur;

    let image_bytes = include_bytes!("../assets/cballs.png");
    let png_data = image::load_from_memory_with_format(image_bytes, image::ImageFormat::Png).expect("couldn't load PNG from memory");
    let width = png_data.width() as usize;
    let height = png_data.height() as usize;
    let data = png_data.as_mut_rgb8().unwrap().into_raw();
    let samples: &mut [[u8; 3]] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() / 3) };

    b.iter(|| { gaussian_blur(&mut samples, width as usize, height as usize, 50.0); } );
}
