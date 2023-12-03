use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct GameConfig {
    red: i32,
    green: i32,
    blue: i32,
}

struct DiceRoll {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>
}

fn main() {

    let config = GameConfig{red: 12, green: 13, blue: 14};
    let total = sum_valid_ids("day2.txt", config);
    println!("Valid game ID total: {}", total);
}

fn sum_valid_ids(filename: &str, config: GameConfig) -> u64 {

    let mut sum = 0u64;

    for line in read_lines(filename) {
        let safe_line = line.unwrap();
        let (game_piece, dice_piece) = safe_line.split_once(":").unwrap();
        let game_id_as_num = parse_game_id(game_piece);

        let is_valid = is_game_valid(&config, dice_piece);
        if is_valid {
            sum += game_id_as_num;
        } else {
            println!("Invalid game ID : {}", game_id_as_num);
        }
    }

    return sum;
}

fn parse_game_id(name: &str) -> u64 {

    let (_, id_str) = name.split_once(" ").unwrap();
    return id_str.parse::<u64>().unwrap();
}

fn is_game_valid(config: &GameConfig, line: &str) ->  bool {

    let dice_rolls = parse_dice(line);
    for roll in dice_rolls {
        if roll.red.is_some() && roll.red.unwrap() > config.red {
            return false;
        } else if roll.green.is_some() && roll.green.unwrap() > config.green {
            return false;
        } else if roll.blue.is_some() && roll.blue.unwrap() > config.blue {
            return false;
        }
    }

    return true;
}

fn parse_dice(line: &str) -> Vec<DiceRoll> {

    let mut result = vec!();
    for choice in line.split(";") {
        for dice in choice.split(",") {
            let (temp_value, color) = dice.trim().split_once(" ").unwrap();
            let mut roll = DiceRoll{red: None, green: None, blue: None};
            let value = temp_value.parse::<i32>().unwrap();
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