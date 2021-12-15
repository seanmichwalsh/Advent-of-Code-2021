use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Movement {
    fn new(direction: &str, value: i32) -> Option<Movement> {
        match direction {
            "forward" => Some(Movement::Forward(value)),
            "up" => Some(Movement::Up(value)),
            "down" => Some(Movement::Down(value)),
            _ => None
        }
    }
}

struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Position {
    fn advance(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(x) => self.horizontal += x,
            Movement::Up(y) => self.depth -= y,
            Movement::Down(y) => self.depth += y
        }
    }

    fn advance_aim(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(val) => {
                self.horizontal += val;
                self.depth += self.aim * val;
            },
            Movement::Up(val) => self.aim -= val,
            Movement::Down(val) => self.aim += val,
        }
    }
}

fn part_one(movements: &Vec<Movement>) -> i32 {
    let mut pos = Position{ depth: 0, horizontal: 0, aim: 0 };
    for movement in movements.iter() {
        pos.advance(movement);
    }

    pos.depth * pos.horizontal
}

fn part_two(movements: &Vec<Movement>) -> i32 {
    let mut pos = Position{ depth: 0, horizontal: 0, aim: 0 };
    for movement in movements.iter() {
        pos.advance_aim(movement);
    }

    pos.depth * pos.horizontal
}

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
    let input = File::open("../inputs/day2.txt")?;
    let reader = BufReader::new(input);
    let mut movements: Vec<Movement> = Vec::new();

    for line in reader.lines() {
        let curr_line = line.unwrap();
        let value: Vec<&str> = curr_line.split_whitespace().collect();
        let movement = Movement::new(value[0], value[1].parse::<i32>()?);
        match movement {
            Some(new_movement) => movements.push(new_movement),
            None => return Err("Could not convert input line to valid movement!".into()),
        }
    }

    let part_one = part_one(&movements).to_string();
    let part_two = part_two(&movements).to_string();

    Ok((part_one, part_two))
}