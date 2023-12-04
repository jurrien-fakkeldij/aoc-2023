use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod input;

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;

fn main() {
    println!("Hello, AOC world!");
    let args: Vec<String> = env::args().collect();

    let day = &args[1].parse::<u32>().unwrap();
    let session_id = &args[2];

    match day {
        1 => day1(session_id),
        2 => day2(session_id),
        3 => day3(session_id),
        4 => day4(session_id),
        d => println!("No solution file yet for day {d}"),
    }
}
