/// This file implements a frequency, which simply maps each valid digit (0-9) to a count of how
/// many times it has appeared... somewhere.  For example, a pixel may have a frequency
/// representing how many times the pixel is non-black and its picture contains a particular digit,
/// for some set of pictures.

use common::{Digit, PICTURE_HEIGHT};

pub type Frequency = [usize; 10];

pub fn add(freq: &mut Frequency, d: Digit) {
    freq[d as usize] += 1;
}

pub fn combine(freq: &mut Frequency, other_freq: &Frequency) {
    for i in 0..10 {
        freq[i] += other_freq[i];
    }
}

pub fn mode(freq: &Frequency) -> Digit {
    let mut digit: Digit = 0; 
    let mut max = 0;

    for d in 0..freq.len() {
        if freq[d] >= max {
            digit = d as u8;
            max = freq[d];
        }
    }

    digit
}

pub type Frequencies = Vec<Vec<Frequency>>;

pub fn new_frequencies() -> Frequencies {
    vec![vec![[0; 10]; PICTURE_HEIGHT]; PICTURE_HEIGHT]
}
