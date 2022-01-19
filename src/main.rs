use image::{io::Reader, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb};
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
    let res = make_bw_rbg(img, cutoff);
    let res = draw_edges(res);
    res.save("computer-vision.png")?;
    Ok(())
}

type IB = ImageBuffer<Rgb<u8>, Vec<u8>>;
const WINDOW: u32 = 5;
const MIN_DETECTED: usize = 3;

fn draw_edges(img: IB) -> IB {
    let (w, h) = img.dimensions();
    let ref_img = img;
    let mut img = ImageBuffer::new(w, h);
    for x in 0..w - WINDOW {
        for y in 0..h - WINDOW {
            if is_edge(&ref_img, x, y) {
                let mut si = img.sub_image(x, y, WINDOW, WINDOW);
                for si_x in 0..WINDOW {
                    for si_y in 0..WINDOW {
                        si.put_pixel(si_x, si_y, Rgb::from([255, 0, 0]));
                    }
                }
            }
        }
    }
    img
}

fn is_edge(img: &IB, x: u32, y: u32) -> bool {
    let v = img.view(x, y, WINDOW, WINDOW);
    let mut white = 0;
    let mut black = 0;
    for (_, _, p) in v.pixels() {
        if p.0[0] == 0 {
            black += 1;
        } else {
            white += 1;
        }
    }
    black > MIN_DETECTED && white > MIN_DETECTED
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
