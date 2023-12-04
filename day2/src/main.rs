use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct GameConfig {
    red: u64,
    green: u64,
    blue: u64,
}

struct DiceRoll {
    red: Option<u64>,
    green: Option<u64>,
    blue: Option<u64>
}

fn main() {

    let total = sum_valid_ids("day2.txt");
    println!("Game power total: {}", total);
}

fn sum_valid_ids(filename: &str) -> u64 {

    let mut sum = 0u64;

    for line in read_lines(filename) {
        let safe_line = line.unwrap();
        let (game_piece, dice_piece) = safe_line.split_once(":").unwrap();
        let _ = parse_game_id(game_piece);

        sum += calculate_game_power(dice_piece);
    }

    return sum;
}

fn parse_game_id(name: &str) -> u64 {

    let (_, id_str) = name.split_once(" ").unwrap();
    return id_str.parse::<u64>().unwrap();
}

fn calculate_game_power(line: &str) ->  u64 {

    // This is a bit of a hack since we know the power is a product so using 1 here will not
    // impact the power calculation
    let mut config = GameConfig{red: 1u64, green: 1u64, blue: 1u64};
    let dice_rolls = parse_dice(line);
    for roll in dice_rolls {
        if roll.red.is_some() && roll.red.unwrap() > config.red {
            config.red = roll.red.unwrap();
        } else if roll.green.is_some() && roll.green.unwrap() > config.green {
            config.green = roll.green.unwrap();
        } else if roll.blue.is_some() && roll.blue.unwrap() > config.blue {
            config.blue = roll.blue.unwrap();
        }
    }

    return config.red * config.green * config.blue;
}

fn parse_dice(line: &str) -> Vec<DiceRoll> {

    let mut result = vec!();
    for choice in line.split(";") {
        for dice in choice.split(",") {
            let (temp_value, color) = dice.trim().split_once(" ").unwrap();
            let mut roll = DiceRoll{red: None, green: None, blue: None};
            let value = temp_value.parse::<u64>().unwrap();
            match color {
                "red" => roll.red = Some(value),
                "green" => roll.green = Some(value),
                "blue" => roll.blue = Some(value),
                _ => panic!("Invalid color found"),
            }
            result.push(roll);
        }
    }

    return result;
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}