extern crate csv;
extern crate num;

pub mod common;
pub mod data;
pub mod histogram;

use common::{Digit, DISTANCE, is_shaded, Picture, PICTURE_HEIGHT};
use data::{get_test_data, get_training_data, write_results};
use histogram::{Histograms, Histogram, new_histograms};
use std::env::args;

type Point = (usize, usize);

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
        if !(i < PICTURE_HEIGHT) { break; }

        bound = i + j;
    }

    bound
}

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

fn classify_pixel(index: Point, hists: &Histograms) -> Digit {
    let mut hist = Histogram::new(1);

    for &(x, y) in neighbors(index).iter() {
        hist.combine(&hists[y][x]);
    }

    hist.mode()
}

fn classify(picture: &Picture, hists: &Histograms) -> Digit {
    let mut hist = Histogram::new(1);

    for x in 0..PICTURE_HEIGHT {
        for y in 0..PICTURE_HEIGHT {
            if is_shaded(picture[y][x]) {
                let d = classify_pixel((x, y), &hists);

                hist.add(d);
            }
        }
    }

    hist.mode()
}

fn add_picture(picture: &Picture, result: Digit, hists: &mut Histograms) {
    for x in 0..PICTURE_HEIGHT {
        for y in 0..PICTURE_HEIGHT {
           if is_shaded(picture[y][x]) {
               hists[y][x].add(result);
           }
        }
    }
}

fn train(pictures: &Vec<Picture>, results: &Vec<Digit>) -> Histograms {
    if pictures.len() != results.len() {
        panic!("The number of pictures in the training set did not match the number of results");
    }

    let mut hists: Histograms = new_histograms();

    for i in 0..pictures.len() {
        add_picture(&pictures[i], results[i], &mut hists);
    }

    hists
}

fn test(pictures: &Vec<Picture>, hists: &Histograms) -> Vec<Digit> {
    let mut results: Vec<Digit> = Vec::with_capacity(pictures.len());

    for picture in pictures.iter() {
        results.push(classify(picture, hists));
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

    let hists = train(&training_pictures, &training_results);

    let test_pictures = get_test_data(&test_file);

    let test_results = test(&test_pictures, &hists);

    write_results(&output_file, &test_results);
}
