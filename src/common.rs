pub static DISTANCE: usize = 1;
pub static PICTURE_HEIGHT: usize = 27;

pub type Digit = u8;
pub type Pixel = u8;
pub type Picture = Vec<Vec<Pixel>>;

pub fn new_picture() -> Picture {
    vec![vec![0; PICTURE_HEIGHT]; PICTURE_HEIGHT]
}

pub fn is_not_black(p: Pixel) -> bool {
    p != 0u8
}
