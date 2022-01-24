use image::{GenericImageView, Pixel};
use num_traits::cast::cast;
use std::f64::consts::PI;

// Based on http://www.sydlogan.com/deskew.html

#[derive(Clone)]
struct HoughLine {
    count: usize,
    index: usize,
    alpha: f64,
    d: f64,
}

struct Deskew {
    c_d_min: i32,
    c_alpha_start: f64,
    c_alpha_step: f64,
    c_steps: usize,
    c_d_step: f64,
    c_h_matrix: Vec<i32>,
}

impl HoughLine {
    fn new() -> Self {
        Self {
            count: 0,
            index: 0,
            alpha: 0.0,
            d: 0.0,
        }
    }
}

impl Deskew {
    fn new() -> Self {
        Self {
            c_alpha_start: -20.0,
            c_alpha_step: 0.2,
            c_steps: 40 * 5,
            c_d_step: 1.0,

            // These are placeholders and will be initialized for real in init().
            c_h_matrix: vec![],
            c_d_min: 0,
        }
    }

    fn init<I: GenericImageView>(&mut self, img: &I) {
        let (width, height) = img.dimensions();
        self.c_d_min = -(width as i32);
        let c_d_count = ((2 * (width + height)) as f64 / self.c_d_step) as usize;
        self.c_h_matrix = vec![0; c_d_count * self.c_steps];
    }
}

pub fn get_skew<I: GenericImageView>(img: &I) -> f64 {
    let mut deskew = Deskew::new();
    deskew.init(img);
    deskew.calc(img);
    match deskew.get_top(20) {
        None => 0.0,
        Some(top) => top.iter().map(|hl| hl.alpha).sum::<f64>() / (top.len() as f64),
    }
}

impl Deskew {
    fn get_top(&self, count: usize) -> Option<Vec<HoughLine>> {
        let mut hl = vec![HoughLine::new(); count];

        for (i, val) in self.c_h_matrix.iter().enumerate() {
            if (*val as usize) > hl[count - 1].count {
                hl[count - 1].count = *val as usize;
                hl[count - 1].index = i;
                let mut j = count - 1;
                while j > 0 && hl[j].count > hl[j - 1].count {
                    hl.swap(j, j - 1);
                    j -= 1;
                }
            }
        }

        for i in 0..count {
            let d_index = hl[i].index / self.c_steps;
            let alpha_index = hl[i].index - d_index * self.c_steps;
            hl[i].alpha = self.get_alpha(alpha_index);
            hl[i].d = ((d_index as i32) + self.c_d_min) as f64;
        }

        Some(hl)
    }

    fn calc<I: GenericImageView>(&mut self, img: &I) {
        let (width, height) = img.dimensions();
        let h_min = height / 4;
        let h_max = height * 3 / 4;

        for y in h_min..=h_max {
            for x in 1..width {
                if is_black(img, x, y) {
                    if !is_black(img, x, y + 1) {
                        self.calc_point(x, y);
                    }
                }
            }
        }
    }

    fn calc_point(&mut self, x: u32, y: u32) {
        for alpha in 0..self.c_steps {
            let rads = self.get_alpha(alpha) * PI / 180.0;
            let d = (y as f64) * rads.cos() - (x as f64) * rads.sin();
            let d_index = self.calc_d_index(d);
            let index = d_index * self.c_steps + alpha;
            self.c_h_matrix[index] += 1;
        }
    }
}

const BLACK: u8 = 0;

fn is_black<I: GenericImageView>(img: &I, x: u32, y: u32) -> bool {
    let p = img.get_pixel(x, y);
    let l = p.to_luma();
    let v = l.0[0];
    cast::<_, u8>(v).unwrap() == BLACK
}

impl Deskew {
    fn get_alpha(&self, index: usize) -> f64 {
        self.c_alpha_start + (index as f64) * self.c_alpha_step
    }

    fn calc_d_index(&self, d: f64) -> usize {
        (d - (self.c_d_min as f64)) as usize
    }
}
