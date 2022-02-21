use opencv::core::{Mat, Point, Vec3i, BORDER_DEFAULT};
use opencv::types::VectorOfVec3i;
use opencv::{imgcodecs, imgproc};
use std::env;
use std::error::Error;

mod wordsearch;

// detect dice: https://stackoverflow.com/questions/55169645/square-detection-in-image
// opencv rust: https://docs.rs/opencv/0.62.0/opencv/index.html

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        None => println!("Usage: boggle-ocr IMAGE"),
        Some(arg) => dump(&arg).unwrap(),
    };
}

fn dump(path: &str) -> Result<(), Box<dyn Error>> {
    if path.ends_with(".txt") {
        return wordsearch::find_all_in_file(path);
    }

    let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;

    println!("converting to grayscale...");
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    println!("blurring...");
    let mut blur = Mat::default();
    imgproc::median_blur(&gray, &mut blur, 5)?;
    println!("sharpening...");
    let sharpen_kernel = VectorOfVec3i::from_slice(&[
        Vec3i::from([-1, -1, -1]),
        Vec3i::from([-1, 9, -1]),
        Vec3i::from([-1, -1, -1]),
    ]);
    let mut sharpen = Mat::default();
    imgproc::filter_2d(
        &blur,
        &mut sharpen,
        -1,
        &sharpen_kernel,
        Point { x: -1, y: -1 },
        0.0,
        BORDER_DEFAULT,
    )?;

    println!("applying a threshold...");
    let mut thresh = Mat::default();
    imgproc::threshold(
        &sharpen,
        &mut thresh,
        160.0,
        255.0,
        imgproc::THRESH_BINARY_INV,
    )?;
    /*
    println!("morphology thing...");
    // thresh = thresh[1] // ????
    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_RECT,
        Size {
            width: 3,
            height: 3,
        },
        Point { x: -1, y: -1 },
    )?;
    let mut close = Mat::default();
    imgproc::morphology_ex(
        &thresh,
        &mut close,
        imgproc::MORPH_CLOSE,
        kernel,
        Point { x: -1, y: -1 },
        2,
        imgproc::BORDER_CONSTANT,
        imgproc::morphology_default_border_value()?,
    )?;

    println!("find contours...");
    let mut cnts = Mat::default();
    imgproc::find_contours(
        &close,
        &mut cnts,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(),
    )?;
    // cnts = cnts[0] if len(cnts) == 2 else cnts[1]
    */

    /*
    min_area = 100
    max_area = 1500
    image_number = 0
    for c in cnts:
        area = cv2.contourArea(c)
        if area > min_area and area < max_area:
            x,y,w,h = cv2.boundingRect(c)
            ROI = image[y:y+h, x:x+w]
            cv2.imwrite('ROI_{}.png'.format(image_number), ROI)
            cv2.rectangle(image, (x, y), (x + w, y + h), (36,255,12), 2)
            image_number += 1

    cv2.imshow('sharpen', sharpen)
    cv2.imshow('close', close)
    cv2.imshow('thresh', thresh)
    cv2.imshow('image', image)
    cv2.waitKey()
         */
    Ok(())
}
