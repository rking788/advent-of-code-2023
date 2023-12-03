use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // let total = sum_calibration_values("test-case.txt");
    let total = sum_calibration_values("day1.txt");
    println!("Calibration total: {}", total);
}

fn sum_calibration_values(filename: &str) -> u64 {

    let mut sum = 0u64;
    let mut lineNum = 1;

    for line in read_lines(filename) {
        println!("Line {:?}", lineNum);
        sum += find_line_calibration_value(line.unwrap()) as u64;
        lineNum += 1;
    }

    return sum;
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn find_line_calibration_value(line: String) -> u32 {

    let mut buf: Vec<char> = vec!();
    let mut tens_digit: Option<u32> = None;
    let mut ones_digit: Option<u32> = None;

    for char in line.chars() {
        let mut temp_digit = char.to_digit(10);
        if temp_digit.is_none() {
            buf.push(char);

            let s = buf.clone().into_iter().collect::<String>();
            println!("Buf so far = {}", s);
            if s == "zero" || s.ends_with("zero") {
                temp_digit = Some(0u32);
            } else if s == "one" || s.ends_with("one") {
                temp_digit = Some(1u32);
            } else if s == "two" || s.ends_with("two") {
                temp_digit = Some(2u32);
            } else if s == "three" || s.ends_with("three") {
                temp_digit = Some(3u32);
            } else if s == "four" || s.ends_with("four") {
                temp_digit = Some(4u32);
            } else if s == "five" || s.ends_with("five") {
                temp_digit = Some(5u32);
            } else if s == "six" || s.ends_with("six") {
                temp_digit = Some(6u32);
            } else if s == "seven" || s.ends_with("seven") {
                temp_digit = Some(7u32);
            } else if s == "eight" || s.ends_with("eight") {
                temp_digit = Some(8u32);
            } else if s == "nine" || s.ends_with("nine") {
                temp_digit = Some(9u32);
            }

            if temp_digit.is_some() {
                buf.clear();
                buf.push(char);
            }
        } else {
            buf.clear();
        }

        if temp_digit.is_some() && tens_digit.is_none() {
            tens_digit = temp_digit;
        } else if temp_digit.is_some() {
            ones_digit = temp_digit;
        }
    }

    println!("Tens = {:?} // ones = {:?}", tens_digit, ones_digit);
    let result = match tens_digit {
        Some(tens) => match ones_digit {
            Some(ones) => 10 * tens + ones,
            None => 10 * tens + tens,
        },
        _ => 0,
    };

    println!("Found result = {}", result);

    return result;
}