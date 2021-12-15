use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn part_one(depths: &Vec<u32>) -> u32 {
    let mut inc_count: u32 = 0;

    for pair in depths.iter().zip(depths.iter().skip(1)) {
        let prev = pair.0;
        let next = pair.1;
        if next > prev {
            inc_count += 1;
        }
    }
    
    return inc_count;
}

fn part_two(depths: &Vec<u32>) -> u32 {
    let mut inc_count: u32 = 0;

    for window in depths.windows(3).zip(depths.windows(3).skip(1)) {
        let prev_sum: u32 = window.0.iter().sum();
        let next_sum: u32 = window.1.iter().sum();
        if next_sum > prev_sum {
            inc_count += 1;
        }
    }

    return inc_count;
}

pub fn solve() -> Result<(String, String), Box<dyn Error>> {
    let input = File::open("../inputs/day1.txt")?;
    let reader = BufReader::new(input);
    let mut depths: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let value = line.unwrap();
        depths.push(value.parse::<u32>()?);
    }

    let part_one = part_one(&depths).to_string();
    let part_two = part_two(&depths).to_string();

    Ok((part_one, part_two))
}