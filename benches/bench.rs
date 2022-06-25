use criterion::{criterion_group, criterion_main, Criterion};

fn bench_blur_image(b: &mut Criterion) {

    use blurslice::gaussian_blur;

    let image_bytes = include_bytes!("../assets/cballs.png");
    let mut png_data = image::load_from_memory_with_format(image_bytes, image::ImageFormat::Png).expect("couldn't load PNG from memory");
    let width = png_data.width() as usize;
    let height = png_data.height() as usize;
    let rgb8 = png_data.as_mut_rgb8().unwrap();
    let mut flat_samples = rgb8.as_flat_samples_mut();
    let data = flat_samples.image_mut_slice().unwrap();
    let mut samples: &mut [[u8; 3]] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() / 3) };

    b.bench_function("radius 50.0", |b| b.iter(|| { gaussian_blur(&mut samples, width as usize, height as usize, 50.0); } ));
    b.bench_function("radius 20.0", |b| b.iter(|| { gaussian_blur(&mut samples, width as usize, height as usize, 20.0); } ));
    b.bench_function("radius  1.5", |b| b.iter(|| { gaussian_blur(&mut samples, width as usize, height as usize, 1.5); } ));
}

criterion_group!(benches, bench_blur_image);
criterion_main!(benches);
