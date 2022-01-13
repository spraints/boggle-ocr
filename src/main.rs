use std::env;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        None => println!("Usage: boggle-ocr IMAGE"),
        Some(arg) => dump(&arg).unwrap(),
    };
}

fn dump(path: &str) -> image::ImageResult<()> {
    let img = image::io::Reader::open(path)?.decode()?;
    match img {
        image::DynamicImage::ImageLuma8(_) => println!("ImageLuma8"),
        image::DynamicImage::ImageLumaA8(_) => println!("ImageLumaA8"),
        image::DynamicImage::ImageRgb8(_) => println!("ImageRgb8"),
        image::DynamicImage::ImageRgba8(_) => println!("ImageRgba8"),
        image::DynamicImage::ImageBgr8(_) => println!("ImageBgr8"),
        image::DynamicImage::ImageBgra8(_) => println!("ImageBgra8"),
        image::DynamicImage::ImageLuma16(_) => println!("ImageLuma16"),
        image::DynamicImage::ImageLumaA16(_) => println!("ImageLumaA16"),
        image::DynamicImage::ImageRgb16(_) => println!("ImageRgb16"),
        image::DynamicImage::ImageRgba16(_) => println!("ImageRgba16"),
    };
    Ok(())
}
