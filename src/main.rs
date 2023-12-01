use std::env;

mod day1;
mod day2;
mod input;

use day1::day1;
use day2::day2;

fn main() {
    println!("Hello, AOC world!");
    let args: Vec<String> = env::args().collect();

    let day = &args[1].parse::<u32>().unwrap();

    match day {
        1 => day1(),
        2 => day2(),
        d => println!("No solution file yet for day {d}"),
    }
}
