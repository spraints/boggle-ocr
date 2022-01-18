use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb};
use std::env;
use std::error::Error;

// https://en.wikipedia.org/wiki/Optical_character_recognition

fn main() {
    let mut args = env::args().skip(1);
    let cutoff = 115; // todo make this a command line arg. Lower number makes more white.
    match args.next() {
        None => println!("Usage: boggle-ocr IMAGE"),
        Some(arg) => dump(&arg, cutoff).unwrap(),
    };
}

fn dump(path: &str, cutoff: u8) -> Result<(), Box<dyn Error>> {
    let img = Reader::open(path)?.decode()?;
    let mut res = make_bw_rbg(img, cutoff);
    draw_edges(&mut res);
    res.save("computer-vision.png")?;
    Ok(())
}

type IB = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn draw_edges(img: &mut IB) {
    let (w, h) = img.dimensions();
    for x in 0..w - 1 {
        for y in 0..h - 1 {
            if is_edge(&img, x, y) {
                img.put_pixel(x, y, Rgb::from([255, 0, 0]));
            }
        }
    }
}

fn is_edge(img: &IB, x: u32, y: u32) -> bool {
    let p = img.get_pixel(x, y);
    let r = img.get_pixel(x + 1, y);
    if p != r {
        return true;
    }
    let d = img.get_pixel(x, y + 1);
    if p != d {
        return true;
    }
    false
}

fn make_bw_rbg(img: DynamicImage, cutoff: u8) -> IB {
    let (w, h) = img.dimensions();
    let mut res = ImageBuffer::new(w, h);
    for (x, y, p) in img.pixels() {
        let mut bw = p.to_luma();
        if bw.0[0] > cutoff {
            bw.0[0] = 255;
        } else {
            bw.0[0] = 0;
        }
        res.put_pixel(x, y, bw.to_rgb());
    }
    res
}
