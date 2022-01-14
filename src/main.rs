use image::{GenericImageView, Pixel};
use std::env;
use std::error::Error;
use std::fs::File;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        None => println!("Usage: boggle-ocr IMAGE"),
        Some(arg) => dump(&arg).unwrap(),
    };
}

fn dump(path: &str) -> Result<(), Box<dyn Error>> {
    let img = image::io::Reader::open(path)?.decode()?;
    let encoder = image::codecs::png::PngEncoder::new(File::create("grayscale.png")?);
    let (x, y) = img.dimensions();
    let cutoff = 115; // todo make this a command line arg. Lower number makes more white.
    encoder.encode(&lumas(&img, cutoff), x, y, image::ColorType::L8)?;
    Ok(())
}

fn lumas(img: &image::DynamicImage, cutoff: u8) -> Vec<u8> {
    img.pixels().map(|(_, _, p)| bw(p, cutoff)).collect()
}

fn bw<P: Pixel<Subpixel = u8>>(p: P, cutoff: u8) -> u8 {
    let v = p.to_luma().0[0];
    if v > cutoff {
        255
    } else {
        0
    }
}
