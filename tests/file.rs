use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if !(2..=3).contains(&args.len()) {
        let usage = "usage: cargo test -- <imagefile> <radius> [<outfile>]";
        if args.len() == 1 {
            eprintln!("{usage} (assuming running in general test run, returning success)");
            return Ok(());
        }
        return Err(usage.into());
    }
    let filename = &args[1];
    let radius: f32 = args[2].to_str().unwrap().parse()?;
    let outfile = args.get(3).map(std::ffi::OsString::as_os_str).unwrap_or(std::ffi::OsStr::new("blurred-out.png"));

    let mut image = image::io::Reader::open(filename)?.decode()?;
    let width = image.width() as usize;
    let height = image.height() as usize;
    let rgb8 = image.as_mut_rgb8().expect("is RGB8 image");
    let mut flat_samples = rgb8.as_flat_samples_mut();
    let data = flat_samples.image_mut_slice().unwrap();
    let mut samples: &mut [[u8; 3]] = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().cast(), data.len() / 3) };

    eprintln!("blurring {filename:?} at radius {radius}...");
    blurslice::gaussian_blur(&mut samples, width as usize, height as usize, radius);
    image.save(outfile)?;
    Ok(())
}
