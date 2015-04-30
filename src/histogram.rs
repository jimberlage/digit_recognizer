use common::{Digit, PICTURE_HEIGHT};

#[derive(Clone)]
pub struct Histogram {
    data: [usize; 10],
    weight: usize,
}

impl Histogram {
    pub fn add(&mut self, d: Digit) {
        self.data[d as usize] += 1;
    }

    pub fn combine(&mut self, other_hist: &Histogram) {
        for i in 0..10 {
            self.data[i] += other_hist.data[i];
        }
    }

    pub fn mode(&self) -> Digit {
        let mut digit: Digit = 0; 
        let mut max = 0;

        for d in 0..10 {
            if self.data[d] >= max {
                digit = d as u8;
                max = self.data[d];
            }
        }

        digit
    }

    pub fn new(weight: usize) -> Histogram {
        Histogram {
            data: [0; 10],
            weight: weight,
        }
    }

    pub fn total(&self) -> usize {
        let mut result = 0;

        for i in 0..10 {
            result += self.data[i];
        }

        result
    }
}

pub type Histograms = Vec<Vec<Histogram>>;

pub fn new_histograms() -> Histograms {
    vec![vec![Histogram::new(1); PICTURE_HEIGHT]; PICTURE_HEIGHT]
}
