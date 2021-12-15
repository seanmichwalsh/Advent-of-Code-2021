mod day1;
mod day2;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Please enter the day to solve for!".into());
    }

    let day = &args[1].parse::<u32>()?;

    let result = match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3..=25 => return Err("Day not yet implemented!".into()),
        _ => return Err("Advent of Code only runs from Dec 1-25!".into()),
    };

    let answer = result?;
    println!("Day {} - Part 1: {}", day, answer.0);
    println!("Day {} - Part 2: {}", day, answer.1);

    Ok(())
}
