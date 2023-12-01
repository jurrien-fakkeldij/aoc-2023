use crate::input::*;

pub fn day2() {
    println!("Day 2");
    let input = lines_from_day(2);
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<String>) {
    println!("Part 1");
    let test_input = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];
    let output = 0;
    println!("Test: {}", output);
    println!("Output: {}", output);
}

fn part2(input: Vec<String>) {
    println!("Part 2");
    let test_input = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
    ];
    let output = 0;
    println!("Test: {}", output);
    println!("Output: {}", output);
}
