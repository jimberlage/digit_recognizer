pub type Digit = u8;
pub type Pixel = u8;
pub type Picture = [Pixel; 784];

pub trait Classifier {
    fn classify(&self, p: &Picture) -> Option<Digit>;
}

pub trait DigitComparer {
    fn add(&mut self, d: Digit);

    fn mode(&self) -> Option<Digit>;
}

pub fn is_not_black(p: Pixel) -> bool {
    p != 0u8
}
