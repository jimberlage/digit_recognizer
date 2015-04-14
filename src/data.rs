use common::{Digit, Picture};
use csv::{Reader, Result, Writer};
use std::fs::File;
use num::pow;

fn string_to_digit(s: &str) -> u8 {
    let mut result = 0u8;
    let mut i = 0;

    for c in s.chars() {
        let char_as_u8 = c as u8;

        if char_as_u8 > 57u8 || char_as_u8 < 48u8 {
            panic!("Expected a number representation, but got something else");
        }

        let multiplier = pow(10u8, s.len() - i - 1);

        let n = (char_as_u8 - 48u8) * multiplier;

        result += n;
        i += 1;
    }

    result
}

fn read_csv(filename: &str) -> Reader<File> {
    match Reader::from_file(filename) {
        Ok(r) => r.has_headers(true),
        _     => panic!("We couldn't open the file"),
    }
}

fn write_csv(filename: &str) -> Writer<File> {
    match Writer::from_file(filename) {
        Ok(r) => r,
        _     => panic!("We couldn't open the file"),
    }
}

fn convert_potential_row(pr: Result<Vec<String>>, expected_length: usize) -> Vec<String> {
    let row = match pr {
        Ok(r) => r,
        _     => panic!("We have a malformed row"),
    };

    if row.len() != expected_length {
        panic!("We have a malformed row");
    }

    row
}

pub fn get_test_data(filename: &str) -> Vec<Picture> {
    let mut pictures: Vec<Picture> = Vec::new();

    let mut reader = read_csv(filename);

    for potential_row in reader.records() {
        let row = convert_potential_row(potential_row, 784);

        let mut picture = [0u8; 784];

        for i in 0..784 {
            picture[i] = string_to_digit(&row[i]);
        }

        pictures.push(picture);
    }

    pictures
}

pub fn get_training_data(filename: &str) -> (Vec<Picture>, Vec<Digit>) {
    let mut pictures: Vec<Picture> = Vec::new();
    let mut results: Vec<Digit> = Vec::new();

    let mut reader = read_csv(filename);

    for potential_row in reader.records() {
        let row = convert_potential_row(potential_row, 785);

        results.push(string_to_digit(&row[0]));

        let mut picture = [0u8; 784];

        for i in 1..785 {
            picture[i - 1] = string_to_digit(&row[i]);
        }

        pictures.push(picture);
    }

    (pictures, results)
}

pub fn write_results(filename: &str, results: &Vec<Option<Digit>>) {
    let mut writer = write_csv(filename);
    let mut i: usize = 1;

    let r = writer.encode(vec!["ImageId", "Label"]);
    assert!(r.is_ok());

    for &potential_digit in results.iter() {
        let r = match potential_digit {
            Some(n) => writer.encode(vec![i, n as usize]),
            _       => writer.encode(vec![i, 1]),
        };

        assert!(r.is_ok());

        i += 1;
    }
}
