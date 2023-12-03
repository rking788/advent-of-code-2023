use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // let total = sum_calibration_values("test-case.txt");
    let total = sum_calibration_values("part1.txt");
    println!("Calibration total: {}", total);

}

fn sum_calibration_values(filename: &str) -> u64 {

    let mut sum = 0u64;

    for line in read_lines(filename) {
        sum += find_line_calibration_value(line.unwrap()) as u64;
    }

    return sum;
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn find_line_calibration_value(line: String) -> u32 {

    let mut tens_digit: Option<u32> = None;
    let mut ones_digit: Option<u32> = None;

    for char in line.chars() {
        let temp_digit = char.to_digit(10);
        if temp_digit.is_none() {
            continue;
        }

        if tens_digit.is_none() {
            tens_digit = temp_digit;
        }  else {
            ones_digit = temp_digit;
        }
    }

    let result = match tens_digit {
        Some(tens) => match ones_digit {
            Some(ones) => 10 * tens + ones,
            None => 10 * tens + tens,
        },
        _ => 0,
    };

    return result;
}