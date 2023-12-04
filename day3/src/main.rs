use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct NumberLoc {
    value: u64,
    start: Coordinate,
    end: Coordinate,
}

type Coordinate = (i32, i32);

fn main() {

    let sum = sum_part_numbers("input.txt");
    println!("Sum = {}", sum);
}

fn sum_part_numbers(filename: &str) -> u64 {

    let mut all_number_locations: Vec<NumberLoc> = vec!();
    let mut all_symbol_locations: HashMap<i32, HashMap<i32, ()>> = HashMap::new();
    let mut row = 0;

    for line in read_lines(filename) {
        let safe_line = line.unwrap();
        let (mut number_locations, symbol_locations) = parse_line(&safe_line, row);

        all_number_locations.append(&mut number_locations);

        for loc in symbol_locations {
            let y_locs = all_symbol_locations.entry(loc.0).or_insert(HashMap::new());
            y_locs.insert(loc.1, ());
        }

        row += 1;
    }

    // println!("Number locations: {:?}", all_number_locations);
    // println!("Symbol locations: {:?}", all_symbol_locations);

    let mut sum = 0u64;
    for potential in all_number_locations {
        let is_valid = is_valid_part_number(&all_symbol_locations, potential.start, potential.end);
        if is_valid {
            sum += potential.value;
        }
    }

    return sum;
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn parse_line(line: &str, row: i32) -> (Vec<NumberLoc>, Vec<Coordinate>) {

    let mut numbers = vec!();
    let mut symbols = vec!();

    let pattern = Regex::new(r"(\d+)").unwrap();
    for capture in pattern.captures_iter(line){
        let cap = capture.get(0).unwrap();
        let val = cap.as_str().parse::<u64>().unwrap();
        numbers.push(NumberLoc{value: val, start: (row, cap.start() as i32), end: (row, (cap.end() - 1) as i32)});
    }

    for (col, char) in line.chars().enumerate() {
        if !char.is_digit(10) && char != '.' {
            symbols.push((row, col as i32));
        }
    }

    return (numbers, symbols);
}

fn is_valid_part_number(symbols: &HashMap<i32, HashMap<i32, ()>>, start: Coordinate, end: Coordinate) -> bool {

    // Check left
    if start.1 != 0 {
        if is_symbol_at_coordinate(symbols, start.0, start.1 - 1) {
            println!("Found symbol on left ({}, {})", start.0, start.1 - 1);
            return true;
        }
    }

    // Check right
    if is_symbol_at_coordinate(symbols, end.0, end.1 + 1) {
        println!("Found symbol on right ({}, {})", end.0, end.1 + 1);
        return true;
    }

    // Check above
    for col in (start.1 - 1)..=(end.1+1) {
        if is_symbol_at_coordinate(symbols, start.0 - 1, col) {
            println!("Found symbol above ({}, {})", start.0 - 1, col);
            return true;
        }
    }

    // Check below
    for col in (start.1 - 1)..=(end.1+1) {
        if is_symbol_at_coordinate(symbols, start.0 + 1, col) {
            println!("Found symbol below ({}, {})", start.0 + 1, col);
            return true;
        }
    }

    return false;
}

fn is_symbol_at_coordinate(symbols: &HashMap<i32, HashMap<i32, ()>>, row: i32, col: i32) -> bool {
    if let Some(temp) = symbols.get(&row) {
        if temp.get(&col).is_some() {
            return true;
        }
    }

    return false;
}