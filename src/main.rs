extern crate csv;
extern crate num;
extern crate rand;

pub mod common;
pub mod data;
pub mod decision_tree;
pub mod frequency;

use common::{Classifier, Digit, DigitComparer, Picture, is_not_black};
use data::{get_test_data, get_training_data, write_results};
use decision_tree::{DecisionTree, new_decision_tree};
use frequency::Frequency;
use std::collections::HashMap;
use std::env::args;

fn modes(pictures: &Vec<Picture>, results: &Vec<Digit>) -> HashMap<usize, Digit> {
    if pictures.len() != results.len() {
        panic!("The number of pictures does not match the number of results");
    }

    let mut frequencies: [Frequency; 784] = [[None; 10]; 784];

    for i in 0..pictures.len() {
        let picture = pictures[i];
        let result = results[i];

        for j in 0..picture.len() {
            let pixel = picture[j];

            if is_not_black(pixel) {
                frequencies[pixel as usize].add(result);
            }
        }
    }

    let mut result: HashMap<usize, Digit> = HashMap::with_capacity(784);

    for i in 0..frequencies.len() {
        let mode = frequencies[i].mode();

        if mode.is_some() {
            result.insert(i, mode.unwrap());
        }
    }

    result
}

fn create_decision_trees(pictures: &Vec<Picture>, results: &Vec<Digit>) -> Vec<DecisionTree> {
    let num_trees = 10000;
    let m: HashMap<usize, Digit> = modes(pictures, results);
    let mut result: Vec<DecisionTree> = Vec::with_capacity(num_trees);

    for _ in 0..num_trees {
        result.push(new_decision_tree(&m));
    }

    result
}

fn classify(pictures: &Vec<Picture>, d_trees: &Vec<DecisionTree>) -> Vec<Option<Digit>> {
    let mut result: Vec<Option<Digit>> = Vec::with_capacity(pictures.len());

    for pic in pictures.iter() {
        let mut freq: Frequency = [None; 10];

        for tree in d_trees.iter() {
            let classification = tree.classify(pic);

            if classification.is_some() {
                freq.add(classification.unwrap());
            }
        }

        result.push(freq.mode());
    }

    result
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

    let decision_trees = create_decision_trees(&training_pictures, &training_results);

    let test_pictures = get_test_data(&test_file);

    let test_results = classify(&test_pictures, &decision_trees);

    write_results(&output_file, &test_results);
}
