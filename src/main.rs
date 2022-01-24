use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb};
use skew::get_skew;
use std::env;
use std::error::Error;
use std::f64::consts::PI;

mod skew;

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
    let res = make_bw_rbg(img, cutoff);
    let angle = get_skew(&res);
    println!("angle: {}", angle);
    let res = draw_angle(res, angle);
    res.save("computer-vision.png")?;
    Ok(())
}

type IB = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn draw_angle(mut img: IB, angle: f64) -> IB {
    // +
    // |\  <-- angle
    // | \
    // |  \
    // +   + <-- x / y = tan(angle)
    let (w, h) = img.dimensions();
    let mid_x = w / 2;
    let angle = -angle * PI / 180.0;
    let tan_angle = angle.tan();
    println!("shift: {}", tan_angle);
    for y in 0..h {
        let x = mid_x + (y as f64 * tan_angle) as u32;
        for x in x..x + 3 {
            img.put_pixel(x, y, Rgb::from([255, 0, 0]))
        }
    }
    img
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
