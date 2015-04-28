extern crate csv;
extern crate num;

pub mod common;
pub mod data;
pub mod frequency;

use common::{Digit, DISTANCE, is_not_black, Picture, PICTURE_HEIGHT};
use data::{get_test_data, get_training_data, write_results};
use frequency::{add, combine, Frequencies, Frequency, mode, new_frequencies};
use std::env::args;

type Point = (usize, usize);

fn in_bounds(i: usize) -> bool {
    i < PICTURE_HEIGHT
}

fn lower_bound(i: usize) -> usize {
    let mut bound = i;

    for j in 1..(DISTANCE + 1) {
        match i.checked_sub(j) {
            Some(b) => { bound = b; },
            None    => { break; },
        };
    }

    bound
}

fn upper_bound(i: usize) -> usize {
    let mut bound = i;

    for j in 1..(DISTANCE + 1) {
        if !in_bounds(i + j) { break; }

        bound = i + j;
    }

    bound
}

/// This function returns the k nearest neighbors to the point (x, y).
fn neighbors(index: Point) -> Vec<Point> {
    let (x, y) = index;
    let mut results: Vec<Point> = Vec::with_capacity(8);

    for xi in lower_bound(x)..upper_bound(x) {
        for yi in lower_bound(y)..upper_bound(y) {
            results.push((xi, yi));
        }
    }

    results
}

fn classify_pixel(index: Point, freqs: &Frequencies) -> Digit {
    let mut f: Frequency = [0; 10];

    for &(x, y) in neighbors(index).iter() {
        combine(&mut f, &freqs[y][x]);
    }

    mode(&f)
}

fn classify(picture: &Picture, freqs: &Frequencies) -> Digit {
    let mut f: Frequency = [0; 10];

    for x in 0..PICTURE_HEIGHT {
        for y in 0..PICTURE_HEIGHT {
            if is_not_black(picture[y][x]) {
                let d = classify_pixel((x, y), &freqs);

                add(&mut f, d);
            }
        }
    }

    mode(&f)
}

fn add_picture(picture: &Picture, result: Digit, freqs: &mut Frequencies) {
    for x in 0..PICTURE_HEIGHT {
        for y in 0..PICTURE_HEIGHT {
           if is_not_black(picture[y][x]) {
               add(&mut freqs[y][x], result);
           }
        }
    }
}

fn train(pictures: &Vec<Picture>, results: &Vec<Digit>) -> Frequencies {
    if pictures.len() != results.len() {
        panic!("The number of pictures in the training set did not match the number of results");
    }

    let mut freqs: Frequencies = new_frequencies();

    for i in 0..pictures.len() {
        add_picture(&pictures[i], results[i], &mut freqs);
    }

    freqs
}

fn test(pictures: &Vec<Picture>, freqs: &Frequencies) -> Vec<Digit> {
    let mut results: Vec<Digit> = Vec::with_capacity(pictures.len());

    for picture in pictures.iter() {
        results.push(classify(picture, freqs));
    }

    results
}

fn main() {
    let training_file = match args().nth(1) {
        Some(arg) => arg,
        _         => panic!("You must supply a training file"),
    };

    let test_file = match args().nth(2) {
        Some(arg) => arg,
        _         => panic!("You must supply a test file"),
    };

    let output_file = match args().nth(3) {
        Some(arg) => arg,
        _         => panic!("You must supply an output file"),
    };

    let (training_pictures, training_results) = get_training_data(&training_file);

    let freqs = train(&training_pictures, &training_results);

    let test_pictures = get_test_data(&test_file);

    let test_results = test(&test_pictures, &freqs);

    write_results(&output_file, &test_results);
}
