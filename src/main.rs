use image::GenericImageView;
use image::Pixel;
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
    //let img = img.grayscale();
    let encoder = image::codecs::png::PngEncoder::new(File::create("grayscale.png")?);
    let (x, y) = img.dimensions();
    encoder.encode(&lumas(&img), x, y, image::ColorType::L8)?;
    Ok(())
}

fn lumas(img: &image::DynamicImage) -> Vec<u8> {
    img.pixels().map(|(_, _, p)| p.to_luma().0[0]).collect()
}
