/// This file implements a frequency, which simply maps each valid digit (0-9) to a count of how
/// many times it has appeared... somewhere.  For example, a pixel may have a frequency
/// representing how many times the pixel is non-black and its picture contains a particular digit,
/// for some set of pictures.
///
/// A Frequency implements DigitComparer, which means a digit may be added to its internal count,
/// and the mode may be taken (returning whichever digit is represented most often in the 
/// frequency). It's possible that a digit never appears.

use common::{Digit, DigitComparer};

pub type Frequency = [Option<usize>; 10];

impl DigitComparer for Frequency {
    fn add(&mut self, d: Digit) {
        self[d as usize] = match self[d as usize] {
            Some(count) => Some(count + 1usize),
            None        => Some(1usize),
        }
    }

    fn mode(&self) -> Option<Digit> {
        let mut digit = None;
        let mut max = 0;

        for d in 0..self.len() {
            match self[d] {
                Some(count) if count >= max => { digit = Some(d as u8); max = count },
                _                           => {},
            };
        }

        digit
    }
}
