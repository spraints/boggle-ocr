use image::GenericImageView;

#[derive(Clone)]
struct HoughLine {
    count: usize,
    index: usize,
    alpha: f64,
    d: f64,
}

struct Skew {
    d_min: i32,
    alpha_start: f64,
    alpha_step: f64,
    steps: usize,
    d_step: f64,
    matrix: Vec<i32>,
}

pub fn get_skew<I: GenericImageView>(img: &I) -> f64 {
    let (w, h) = img.dimensions();

    let d_count = 2 * (w + h);
    let mut skew = Skew {
        alpha_start: -20.0,
        alpha_step: 0.2,
        steps: 40 * 5,
        d_step: 1.0,
        matrix: None,
        d_min: -w,
        matrix: vec![0; d_count * 40 * 5],
    };

    calc(&mut skew, &img);

    let top = match get_top(&skew, 20) {
        None => return 0.0,
        Some(x) => x,
    };

    top.iter().map(|hl| hl.alpha).sum() / top.len()
}

fn get_top(skew: &Skew, count: usize) -> Vec<HoughLine> {
    let mut hl = vec![
        HoughLine {
            count: 0,
            index: 0,
            alpha: 0.0,
            d: 0.0
        };
        count
    ];
    for (i, val) in skew.matrix.enumerate() {
        if val > hl[count - 1].count {
            hl[count - 1].count = val;
            hl[count - 1].index = i;
            let j = count - 1;
            while j > 0 && hl[j].count > hl[j - 1].count {
                hl.swap(j, j - 1);
                j -= 1;
            }
        }
    }
    let mut d_index = 0;
    let mut alpha_index = 0;
    for i in 0..count {
        d_index = hl[i].index / skew.steps;
        alpha_index = hl[i].index - d_index * skew.steps;
        hl[i].alpha = get_alpha(skew, alpha_index);
        hl[i].d = d_index + skew.d_min;
    }

    hl
}

fn calc<I: GenericImageView>(skew: &mut Skew, img: &I) {
    let mut last_row = None;
    for y in 0..(h - 1) {
        let cur_row = (0..w).map(|x| is_black(img.get_pixel(x, y))).collect();
        if let Some(last_row) = last_row {
            for (top, bottom) in zip(&cur_row, last_row) {
                if top && !bottom {
                    calc(x, y);
                }
            }
        }
        last_row = Some(cur_row);
    }
}
