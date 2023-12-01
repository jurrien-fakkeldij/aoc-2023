use std::env;

mod day1;
mod input;

fn main() {
    println!("Hello, AOC world!");
    let args: Vec<String> = env::args().collect();

    let day = &args[1].parse::<u32>().unwrap();

    match day {
        1 => day1::day1(),
        d => println!("No solution file yet for day {d}"),
    }
}
